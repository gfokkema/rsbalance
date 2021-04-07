extern crate config;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod error;
mod loadbalancer;
mod settings;

use tokio::net::TcpListener;
use tokio::signal;

use error::Result;
use settings::Settings;

#[tokio::main]
pub async fn main() -> Result<()> {
    tracing_subscriber::fmt::try_init()?;

    let settings = Settings::new()?;
    let listener = TcpListener::bind((&settings.frontend.addr[..], settings.frontend.port)).await?;

    tracing::info!("Starting...");
    loadbalancer::run(listener, settings, signal::ctrl_c()).await?;
    tracing::info!("Ending...");

    Ok(())
}
