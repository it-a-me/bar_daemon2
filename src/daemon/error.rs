use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
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
    #[error("error when calling command '{0}'")]
    Command(#[from] crate::command::Error),
    #[error("invalid rocm-smi json")]
    RocmSmi,
}
