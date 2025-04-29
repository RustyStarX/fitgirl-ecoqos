use std::io;

use thiserror::Error;
use wmi::WMIError;

pub mod config;
pub mod listen;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to load config: {0}")]
    InitConfigFailed(&'static str),
    #[error("IO: {0}")]
    IOError(#[from] io::Error),
    #[error("Toml deserialization: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("WMI: {0}")]
    WMI(#[from] WMIError),
}
