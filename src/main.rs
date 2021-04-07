extern crate config;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod error;
mod settings;
mod loadbalancer;

use error::Result;
use settings::Settings;
use loadbalancer::LoadBalancer;

fn main() -> Result<()> {
    let settings = Settings::new()?;
    let loadbalancer = LoadBalancer::new(settings)?;
    loadbalancer.run()?;

    Ok(())
}
