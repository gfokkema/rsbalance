use rsbalance::error::Result;
use rsbalance::loadbalancer;
use rsbalance::settings::Settings;

use tokio::net::TcpListener;
use tokio::signal;

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
