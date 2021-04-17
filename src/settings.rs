use config::{Config, ConfigError, Environment, File};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Frontend {
    pub addr: String,
    pub backend: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub addr: String,
}

#[derive(Debug, Deserialize)]
pub struct Backend {
    pub servers: Vec<Server>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub frontends: HashMap<String, Frontend>,
    pub backends: HashMap<String, Backend>,
    pub debug: bool,
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
