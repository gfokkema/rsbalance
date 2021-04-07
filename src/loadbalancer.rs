use crate::error::Result;
use crate::settings::Settings;

use std::future::Future;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{sleep, Duration};
use tracing::{error, info};

async fn process(socket: TcpStream) {
    info!("processing {:?}", socket);
    tokio::time::sleep(Duration::from_millis(5000)).await;
    info!("finished {:?}", socket);
}

async fn listen(listener: TcpListener) -> Result<()> {
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move { process(socket) });
    }
}

pub async fn run(listener: TcpListener, _settings: Settings, shutdown: impl Future) -> Result<()> {
    tokio::select! {
        res = listen(listener) => {
            match res {
                Err(err) => Err(err),
                Ok(_) => Ok(()),
            }
        }
        _ = shutdown => {
            info!("shutting down");
            Ok(())
        }
    }
}
