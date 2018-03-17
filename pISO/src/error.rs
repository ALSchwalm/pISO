use std::io;
use sysfs_gpio;

#[derive(Debug, ErrorChain)]
pub enum ErrorKind {
    #[error_chain(foreign)]
    Gpio(sysfs_gpio::Error),

    #[error_chain(foreign)]
    Io(io::Error),

    Msg(String),
}
