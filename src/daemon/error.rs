use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Network(#[from] Network),
    #[error("{0}")]
    Fs(#[from] Fs),
    #[error("error when calling command '{0}'")]
    Command(#[from] crate::command::Error),
    #[error("failed to parse {0} to number")]
    ParseNumber(String),
    #[error("invalid rocm-smi json")]
    RocmSmi,
}

#[derive(Error, Debug)]
pub enum Network {
    #[error("failed to initalize tokio runtime, panicing")]
    Runtime(std::io::Error),
    #[error("failed to bind port {0}")]
    Bind(String, std::io::Error),
    #[error("failed to accept incomming connection")]
    Accept(std::io::Error),
    #[error("failed to read bytes from connection")]
    Read(std::io::Error),
    #[error("failed to write bytes to connection")]
    Write(std::io::Error),
}

#[derive(Error, Debug)]
pub enum Fs {
    #[error("failed to read bytes from {0}")]
    Read(std::path::PathBuf, std::io::Error),
    #[error("failed to write bytes from {0}")]
    Write(std::path::PathBuf, std::io::Error),
}
