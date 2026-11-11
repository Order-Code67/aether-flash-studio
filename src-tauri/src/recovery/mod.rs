use crate::device_detection::{DetectedDevice, DeviceStatus, DeviceType};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecoveryDiagnosis {
    pub scenario: String,
    pub severity: String,
    pub summary: String,
    pub steps: Vec<String>,
    pub recommended_action: String,
}

pub fn diagnose(device: &DetectedDevice) -> RecoveryDiagnosis {
    if device.is_mock && device.bootloader_mode {
        return RecoveryDiagnosis {
            scenario: "mock_bootloader_only".into(),
            severity: "info".into(),
            summary: "Mock device stuck in bootloader for UI testing.".into(),
            steps: vec![
                "This is Mock Device Mode.".into(),
                "Do not treat recovery actions as real flashing.".into(),
            ],
            recommended_action: "Use real hardware for recovery validation.".into(),
        };
    }

    if device.bootloader_mode || device.status == DeviceStatus::Bootloader {
        return RecoveryDiagnosis {
            scenario: "bootloader_only".into(),
            severity: "high".into(),
            summary: "Device enumerates in bootloader/download mode.".into(),
            steps: vec![
                "Confirm power and USB cable.".into(),
                "Select a verified compatible firmware artifact.".into(),
                "Run prepare → confirm → flash with verification.".into(),
                "Rescan and confirm application firmware version.".into(),
            ],
            recommended_action: "Flash verified firmware via Flashing Engine".into(),
        };
    }

    if device.device_type == DeviceType::Generic || device.device_type == DeviceType::Unknown {
        return RecoveryDiagnosis {
            scenario: "usb_unclassified".into(),
            severity: "medium".into(),
            summary: "USB/UART device is visible but not classified as RX/TX.".into(),
            steps: vec![
                "Check VID/PID and USB product strings.".into(),
                "Try manual connect with correct baud.".into(),
                "If ExpressLRS, ensure module is in a known mode.".into(),
            ],
            recommended_action: "Identify hardware before flashing".into(),
        };
    }

    if device.status == DeviceStatus::Error {
        return RecoveryDiagnosis {
            scenario: "device_error".into(),
            severity: "high".into(),
            summary: "Device is marked Error.".into(),
            steps: vec![
                "Reconnect USB.".into(),
                "Inspect operation history.".into(),
                "Generate a diagnostic report.".into(),
            ],
            recommended_action: "Run diagnostics then retry".into(),
        };
    }

    RecoveryDiagnosis {
        scenario: "healthy_or_unknown".into(),
        severity: "low".into(),
        summary: "No critical recovery scenario detected from current metadata.".into(),
        steps: vec![
            "Read device information.".into(),
            "Backup configuration before maintenance.".into(),
        ],
        recommended_action: "No recovery flash required".into(),
    }
}
