mod expresslrs;

use crate::device_detection::DetectedDevice;
use expresslrs::ExpressLrsAdapter;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdapterNotSupported {
    pub message: String,
}

#[derive(Debug)]
pub enum AdapterError {
    NotSupported(String),
    Message(String),
}

impl fmt::Display for AdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdapterError::NotSupported(msg) => write!(f, "not supported: {}", msg),
            AdapterError::Message(msg) => write!(f, "{}", msg),
        }
    }
}

pub type AdapterResult<T> = Result<T, AdapterError>;

impl From<AdapterError> for crate::error::AppError {
    fn from(value: AdapterError) -> Self {
        match value {
            AdapterError::NotSupported(msg) => crate::error::AppError::NotSupported(msg),
            AdapterError::Message(msg) => crate::error::AppError::Message(msg),
        }
    }
}

pub trait HardwareAdapter: Send + Sync {
    fn id(&self) -> &'static str;
    fn supports(&self, device: &DetectedDevice) -> bool;
    fn read_configuration(&self, device: &DetectedDevice) -> AdapterResult<serde_json::Value>;
    fn flash_firmware(
        &self,
        _device: &DetectedDevice,
        _artifact_path: &std::path::Path,
    ) -> AdapterResult<()> {
        Err(AdapterError::NotSupported(
            "Adapter flash is handled by the flashing engine (Phase 5)".into(),
        ))
    }
}

pub fn adapter_for(device: &DetectedDevice) -> Option<&'static dyn HardwareAdapter> {
    static ELRS: ExpressLrsAdapter = ExpressLrsAdapter;
    if ELRS.supports(device) {
        Some(&ELRS)
    } else {
        None
    }
}

pub fn is_expresslrs_device(device: &DetectedDevice) -> bool {
    expresslrs::ExpressLrsAdapter::supports_device(device)
}
