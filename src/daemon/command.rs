use crate::error;
use thiserror::Error;
#[derive(Clone)]
pub struct Command {
    command: String,
    args: Vec<String>,
    full_command: String,
}
impl Command {
    pub fn new(command: &str, args: &[&str]) -> Command {
        let command = String::from(command);
        let args: Vec<String> = args.iter().map(|s| String::from(*s)).collect();
        let mut full_command = command.clone();
        for arg in &args {
            full_command.push_str(arg);
        }
        Self {
            command,
            args,
            full_command,
        }
    }

    pub async fn call(&self) -> Result<String, Error> {
        Ok(
            match tokio::process::Command::new(&self.command)
                .args(&self.args)
                .output()
                .await
            {
                Ok(output) => String::from_utf8(output.stdout)
                    .map_err(|e| Error::new(self.full_command.clone(), ErrorType::Utf8(e)))?,
                Err(e) => return Err(Error::new(self.full_command.clone(), ErrorType::Call(e))),
            },
        )
    }
    pub async fn call_as_json(&self) -> Result<json::JsonValue, Error> {
        json::parse(self.call().await?.trim())
            .map_err(|e| Error::new(self.full_command.clone(), ErrorType::Json(e)))
    }
}

#[derive(Error, Debug)]
pub struct Error {
    pub command: String,
    pub error: ErrorType,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} for '{}'", self.error, self.command)
    }
}
impl Error {
    pub fn new(command: String, error: ErrorType) -> Self {
        Self { command, error }
    }
}

#[derive(Error, Debug)]
pub enum ErrorType {
    #[error("parse json '{0}'")]
    Json(#[from] json::Error),
    #[error("{0}")]
    Call(std::io::Error),
    #[error("command returned invalid utf8")]
    Utf8(#[from] std::string::FromUtf8Error),
}
