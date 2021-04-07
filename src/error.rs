use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, HAError>;

pub enum HAError {
    Config(config::ConfigError),
    Net(std::io::Error),
}

impl fmt::Debug for HAError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self)
    }
}

impl fmt::Display for HAError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HAError::Config(ref _e) => write!(f, "Invalid configuration!"),
            HAError::Net(ref _e) => write!(f, "Network error!"),
        }
    }
}

impl From<std::io::Error> for HAError {
    fn from(err: std::io::Error) -> HAError {
        HAError::Net(err)
    }
}

impl From<config::ConfigError> for HAError {
    fn from(err: config::ConfigError) -> HAError {
        HAError::Config(err)
    }
}
