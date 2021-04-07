use config::{ConfigError, Config, File, Environment};

#[derive(Debug, Deserialize)]
pub struct Address {
    pub addr: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Backend {
    pub servers: Vec<Address>
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub frontend: Address,
    pub backend: Backend,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("config/default"))?;
        s.merge(File::with_name("config/local").required(false))?;
        s.merge(Environment::with_prefix("app"))?;
        s.try_into()
    }
}
