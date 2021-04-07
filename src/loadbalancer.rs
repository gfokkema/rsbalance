use std::net::{ToSocketAddrs, SocketAddr};
use std::net::{TcpListener, TcpStream};
use crate::error::Result;
use crate::settings::Settings;

pub struct LoadBalancer {
    listener: TcpListener,
    targets: Vec<SocketAddr>,
}

impl LoadBalancer {
    pub fn new(settings: Settings) -> Result<Self> {
        let lb = LoadBalancer {
            listener: TcpListener::bind((&settings.frontend.addr[..], settings.frontend.port))?,
            targets: settings.backend.servers.iter().flat_map(|s| (&s.addr[..], s.port).to_socket_addrs().unwrap()).collect(),
        };
        println!("{:?}", lb.listener);
        Ok(lb)
    }

    pub fn run(&self) -> Result<()> {
        for stream in self.listener.incoming() {
            handle_client(stream?);
        }
        Ok(())
    }
}

fn handle_client(stream: TcpStream) {
    println!("{:?}", stream);
}
