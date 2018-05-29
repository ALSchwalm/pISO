use serde_json;
use std;
use std::io;
use sysfs_gpio;
use toml;

#[derive(Debug, ErrorChain)]
pub enum ErrorKind {
    #[error_chain(foreign)]
    Json(serde_json::Error),

    #[error_chain(foreign)]
    Toml(toml::de::Error),

    #[error_chain(custom)]
    SyncPoisonError(String),

    #[error_chain(foreign)]
    Gpio(sysfs_gpio::Error),

    #[error_chain(foreign)]
    Io(io::Error),

    Msg(String),
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(err: std::sync::PoisonError<T>) -> Self {
        use std::error::Error;

        Self::from_kind(ErrorKind::SyncPoisonError(err.description().to_string()))
    }
}
