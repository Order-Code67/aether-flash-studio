use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    Message(String),
    #[error("database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serial port error: {0}")]
    Serial(String),
    #[error("port already locked: {0}")]
    PortLocked(String),
    #[allow(dead_code)]
    #[error("device not found: {0}")]
    DeviceNotFound(String),
    #[allow(dead_code)]
    #[error("not supported: {0}")]
    NotSupported(String),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        AppError::Message(value.to_string())
    }
}

impl From<serialport::Error> for AppError {
    fn from(value: serialport::Error) -> Self {
        AppError::Serial(value.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        AppError::Message(value.to_string())
    }
}
