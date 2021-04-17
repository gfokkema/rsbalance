use rsbalance::error::Result;
use rsbalance::loadbalancer;
use rsbalance::settings::Settings;

use tokio::signal;

#[tokio::main]
pub async fn main() -> Result<()> {
    tracing_subscriber::fmt::try_init()?;

    let settings = Settings::new()?;
    tracing::info!("{:#?}", settings);
    let loadbalancer = loadbalancer::LoadBalancer::new(&settings);

    tracing::info!("starting...");
    loadbalancer.run(signal::ctrl_c()).await?;
    tracing::info!("ending...");

    Ok(())
}
