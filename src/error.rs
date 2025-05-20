use std::{env::VarError, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  ConnectionError,
  SocketWriteError,
  SocketReadError,
  SerializationError,
  EnvError(String),
}

impl From<VarError> for Error {
  fn from(error: VarError) -> Self {
    match error {
      VarError::NotPresent => Self::EnvError("Not present".to_string()),
      VarError::NotUnicode(_) => Self::EnvError("Provided os string is not unicode".to_string()),
    }
  }
}

impl From<serde_json::Error> for Error {
  fn from(_: serde_json::Error) -> Self {
    Self::SerializationError
  }
}
