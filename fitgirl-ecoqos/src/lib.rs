use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to load config: {0}")]
    InitConfigFailed(&'static str),
    #[error("Toml deserialization: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Listen Error: {0}")]
    Listen(#[from] listen_new_proc::Error),

    #[cfg(feature = "regex")]
    #[error("invalid regex: {0}")]
    Regex(#[from] regex::Error),
}

pub mod config;
