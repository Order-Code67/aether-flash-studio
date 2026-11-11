use crate::device_detection::{DetectedDevice, DeviceStatus};
use crate::error::{AppError, AppResult};
use crate::flashing::adapters_uart::{esptool_chip_id, resolve_esptool};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BootloaderStatus {
    pub device_id: String,
    pub mode: String,
    pub bootloader_mode: bool,
    pub available: bool,
    pub method: Option<String>,
    pub notes: String,
}

pub fn inspect(device: &DetectedDevice) -> BootloaderStatus {
    let mode = if device.bootloader_mode || device.status == DeviceStatus::Bootloader {
        "bootloader"
    } else {
        "application"
    };
    let esp_like = device
        .mcu
        .as_deref()
        .map(|m| m.to_uppercase().contains("ESP"))
        .unwrap_or(false)
        || device.radio_system.as_deref() == Some("ExpressLRS");

    BootloaderStatus {
        device_id: device.id.clone(),
        mode: mode.into(),
        bootloader_mode: mode == "bootloader",
        available: esp_like && device.com_port.is_some(),
        method: if esp_like {
            Some("esptool chip_id / hard_reset (documented ESP download mode)".into())
        } else {
            None
        },
        notes: if esp_like {
            "Uses documented ESP download-mode entry only when esptool is installed.".into()
        } else {
            "No documented bootloader entry method for this device in Aether Flash Studio yet."
                .into()
        },
    }
}

pub fn enter_bootloader(device: &DetectedDevice) -> AppResult<BootloaderStatus> {
    let mut status = inspect(device);
    if !status.available {
        return Err(AppError::NotSupported(status.notes.clone()));
    }
    if device.is_mock {
        return Err(AppError::Message(
            "Mock devices cannot enter a real bootloader".into(),
        ));
    }
    let port = device
        .com_port
        .clone()
        .ok_or_else(|| AppError::Message("No COM port".into()))?;
    let esptool = resolve_esptool().ok_or_else(|| {
        AppError::Message("esptool not found on PATH — cannot enter ESP download mode".into())
    })?;
    let baud = device.baud_rate.unwrap_or(115200);
    esptool_chip_id(&esptool, &port, baud)?;
    status.mode = "bootloader".into();
    status.bootloader_mode = true;
    status.notes = "esptool chip_id succeeded — device responded in ESP download mode".into();
    Ok(status)
}

pub fn exit_bootloader(device: &DetectedDevice) -> AppResult<BootloaderStatus> {
    let mut status = inspect(device);
    if device.is_mock {
        return Err(AppError::Message(
            "Mock devices cannot exit a real bootloader".into(),
        ));
    }
    let port = device
        .com_port
        .clone()
        .ok_or_else(|| AppError::Message("No COM port".into()))?;
    let esptool = resolve_esptool().ok_or_else(|| {
        AppError::NotSupported("esptool required for documented ESP reset-after-sync".into())
    })?;
    let baud = device.baud_rate.unwrap_or(115200);
    let _ = std::process::Command::new(&esptool.executable)
        .args([
            "--port",
            &port,
            "--baud",
            &baud.to_string(),
            "--after",
            "hard_reset",
            "chip_id",
        ])
        .output()?;
    status.mode = "application".into();
    status.bootloader_mode = false;
    status.notes = "Requested hard_reset via esptool; re-scan to confirm application mode".into();
    Ok(status)
}
