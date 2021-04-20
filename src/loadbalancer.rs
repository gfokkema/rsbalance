use crate::error::Result;
use crate::settings;

use std::collections::HashMap;
use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{self, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tracing::{error, info};

pub struct Server {
    addr: SocketAddr,
}

pub struct Backend {
    servers: Vec<Server>,
}

pub struct Frontend {
    addr: SocketAddr,
    backend: Arc<Backend>,
}

pub struct LoadBalancer {
    frontends: HashMap<String, Arc<Frontend>>,
    backends: HashMap<String, Arc<Backend>>,
}

impl Backend {
    pub fn iter(&self) -> impl std::iter::Iterator<Item = &Server> {
        std::iter::Iterator::cycle(self.servers.iter())
    }
}

impl Frontend {
    async fn listen(&self, shutdown: broadcast::Receiver<()>) -> Result<()> {
        info!("listening...");
        let listener = TcpListener::bind(self.addr).await?;
        let mut iter = self.backend.iter();
        loop {
            let (inbound, _) = listener.accept().await?;
            let addr = iter.next().unwrap().addr;
            tokio::spawn(async move {
                info!("processing {:?}", inbound);
                let outbound = TcpStream::connect(addr).await?;
                info!("connected {:?}", outbound);
                process(inbound, outbound).await
            });
        }
    }
}

impl LoadBalancer {
    pub fn new(settings: &settings::Settings) -> Self {
        let backends: HashMap<String, Arc<Backend>> = settings
            .backends
            .iter()
            .map(|(k, v)| {
                let backend = Backend {
                    servers: v
                        .servers
                        .iter()
                        .map(|v| Server {
                            addr: v.addr.parse().unwrap(),
                        })
                        .collect(),
                };
                (k.clone(), Arc::new(backend))
            })
            .collect();

        let frontends: HashMap<String, Arc<Frontend>> = settings
            .frontends
            .iter()
            .map(|(k, v)| {
                let frontend = Frontend {
                    addr: v.addr.parse().unwrap(),
                    backend: backends.get(&v.backend).unwrap().clone(),
                };
                (k.clone(), Arc::new(frontend))
            })
            .collect();

        LoadBalancer {
            frontends,
            backends,
        }
    }

    pub async fn run(&self, shutdown: impl Future) -> Result<()> {
        info!("starting...");
        let (send_shutdown, _): (broadcast::Sender<()>, _) = broadcast::channel(1);
        for (_, frontend) in self.frontends.iter() {
            let frontend = frontend.clone();
            let recv_shutdown = send_shutdown.subscribe();
            tokio::spawn(async move { frontend.listen(recv_shutdown).await });
        }

        tokio::select! {
            _ = shutdown => {
                tracing::info!("shutting down");
                let _ = send_shutdown.send(());
            }
        }
        info!("ending...");
        Ok(())
    }
}

async fn process(mut inbound: TcpStream, mut outbound: TcpStream) -> Result<()> {
    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = outbound.split();

    let client_to_server = async {
        io::copy(&mut ri, &mut wo).await?;
        wo.shutdown().await
    };

    let server_to_client = async {
        io::copy(&mut ro, &mut wi).await?;
        wi.shutdown().await
    };

    tokio::try_join!(client_to_server, server_to_client)?;

    Ok(())
}
