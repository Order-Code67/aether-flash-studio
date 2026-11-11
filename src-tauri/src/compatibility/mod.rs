use crate::database::FirmwareArtifactRecord;
use crate::device_detection::{DetectedDevice, DeviceType};
use crate::error::AppResult;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompatibilityIssue {
    pub code: String,
    pub severity: String, // "block" | "warn"
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompatibilityReport {
    pub compatible: bool,
    pub issues: Vec<CompatibilityIssue>,
    pub inferred_mcu_family: Option<String>,
    pub artifact_size: u64,
    pub flash_budget: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct TargetProfile {
    pub name: &'static str,
    pub mcu_family: &'static str,
    pub flash_size_bytes: u64,
    pub max_image_bytes: u64,
    pub device_type: DeviceType,
    pub allowed_ext: &'static [&'static str],
}

const KNOWN_TARGETS: &[TargetProfile] = &[
    TargetProfile {
        name: "ESP8285_RX",
        mcu_family: "ESP8285",
        flash_size_bytes: 1024 * 1024,
        max_image_bytes: 512 * 1024,
        device_type: DeviceType::Rx,
        allowed_ext: &["bin"],
    },
    TargetProfile {
        name: "ESP32_RX",
        mcu_family: "ESP32",
        flash_size_bytes: 4 * 1024 * 1024,
        max_image_bytes: 2 * 1024 * 1024,
        device_type: DeviceType::Rx,
        allowed_ext: &["bin"],
    },
    TargetProfile {
        name: "ESP32_TX",
        mcu_family: "ESP32",
        flash_size_bytes: 4 * 1024 * 1024,
        max_image_bytes: 2 * 1024 * 1024,
        device_type: DeviceType::Tx,
        allowed_ext: &["bin"],
    },
];

fn infer_profile(device: &DetectedDevice) -> Option<&'static TargetProfile> {
    let mcu = device.mcu.as_deref().unwrap_or("").to_uppercase();
    let is_8285 = mcu.contains("8285") || mcu.contains("ESP8266");
    let is_esp32 = mcu.contains("ESP32");
    match device.device_type {
        DeviceType::Rx if is_8285 => Some(&KNOWN_TARGETS[0]),
        DeviceType::Rx if is_esp32 => Some(&KNOWN_TARGETS[1]),
        DeviceType::Tx if is_esp32 => Some(&KNOWN_TARGETS[2]),
        DeviceType::Generic | DeviceType::Unknown if is_8285 => Some(&KNOWN_TARGETS[0]),
        DeviceType::Generic | DeviceType::Unknown if is_esp32 => Some(&KNOWN_TARGETS[1]),
        _ => None,
    }
}

pub fn evaluate(
    device: &DetectedDevice,
    artifact: &FirmwareArtifactRecord,
    write_address: Option<u32>,
) -> AppResult<CompatibilityReport> {
    let mut issues = Vec::new();
    let path = Path::new(&artifact.file_path);
    let meta = fs::metadata(path).ok();
    let artifact_size = meta.map(|m| m.len()).unwrap_or(artifact.file_size as u64);

    if device.is_mock {
        issues.push(CompatibilityIssue {
            code: "mock_device".into(),
            severity: "block".into(),
            message: "Mock devices are not flashable".into(),
        });
    }

    if device.com_port.is_none() {
        issues.push(CompatibilityIssue {
            code: "no_com_port".into(),
            severity: "block".into(),
            message: "Device has no COM port".into(),
        });
    }

    let profile = infer_profile(device);
    let inferred = profile.map(|p| p.mcu_family.to_string());
    let flash_budget = profile.map(|p| p.max_image_bytes);

    if profile.is_none() {
        issues.push(CompatibilityIssue {
            code: "unknown_target".into(),
            severity: "block".into(),
            message: "Cannot infer hardware target/MCU family for compatibility check".into(),
        });
    }

    if let Some(p) = profile {
        if device.device_type != DeviceType::Generic
            && device.device_type != DeviceType::Unknown
            && device.device_type != p.device_type
        {
            issues.push(CompatibilityIssue {
                code: "device_type_mismatch".into(),
                severity: "block".into(),
                message: format!(
                    "Device type {:?} does not match target profile {}",
                    device.device_type.as_str(),
                    p.name
                ),
            });
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        if !p.allowed_ext.iter().any(|a| *a == ext) {
            issues.push(CompatibilityIssue {
                code: "format_mismatch".into(),
                severity: "block".into(),
                message: format!("Firmware extension .{ext} not allowed for {}", p.name),
            });
        }

        if artifact_size == 0 {
            issues.push(CompatibilityIssue {
                code: "empty_image".into(),
                severity: "block".into(),
                message: "Firmware image is empty".into(),
            });
        } else if artifact_size > p.max_image_bytes {
            issues.push(CompatibilityIssue {
                code: "image_too_large".into(),
                severity: "block".into(),
                message: format!(
                    "Image size {artifact_size} exceeds budget {} for {}",
                    p.max_image_bytes, p.mcu_family
                ),
            });
        }

        if let Some(addr) = write_address {
            if addr != 0 && p.mcu_family.starts_with("ESP") {
                issues.push(CompatibilityIssue {
                    code: "unexpected_address".into(),
                    severity: "warn".into(),
                    message: format!(
                        "Non-zero write address 0x{addr:X} for ESP target — verify bootloader layout"
                    ),
                });
            }
        }
    }

    if artifact.sha256.trim().is_empty() {
        issues.push(CompatibilityIssue {
            code: "missing_sha".into(),
            severity: "block".into(),
            message: "Artifact SHA-256 missing".into(),
        });
    }

    if !artifact.verified {
        issues.push(CompatibilityIssue {
            code: "unverified_artifact".into(),
            severity: "warn".into(),
            message: "Artifact is not marked verified — validate before flashing".into(),
        });
    }

    let blocked = issues.iter().any(|i| i.severity == "block");
    Ok(CompatibilityReport {
        compatible: !blocked,
        issues,
        inferred_mcu_family: inferred,
        artifact_size,
        flash_budget,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device_detection::{DeviceStatus, DeviceType};
    use tempfile::NamedTempFile;
    use std::io::Write;

    fn device(mcu: &str, dtype: DeviceType) -> DetectedDevice {
        DetectedDevice {
            id: "d1".into(),
            name: "t".into(),
            manufacturer: Some("ExpressLRS".into()),
            device_type: dtype,
            model: Some("test".into()),
            mcu: Some(mcu.into()),
            com_port: Some("COM1".into()),
            baud_rate: Some(460800),
            vid: None,
            pid: None,
            usb_serial: None,
            radio_system: Some("ExpressLRS".into()),
            frequency_band: Some("2.4 GHz".into()),
            firmware_version: None,
            bootloader_version: None,
            connection_mode: None,
            flash_method: Some("UART".into()),
            bootloader_mode: false,
            status: DeviceStatus::Online,
            is_mock: false,
            notes: None,
            profile_id: None,
        }
    }

    #[test]
    fn blocks_oversized_8285_image() {
        let mut file = NamedTempFile::new().unwrap();
        let big = vec![0u8; 600 * 1024];
        file.write_all(&big).unwrap();
        let artifact = FirmwareArtifactRecord {
            id: "a".into(),
            firmware_version_id: "v".into(),
            hardware_target_id: None,
            file_path: file.path().display().to_string(),
            sha256: "abc".into(),
            file_size: big.len() as i64,
            verified: true,
            manufacturer: "ExpressLRS".into(),
            radio_system: "ExpressLRS".into(),
            version: "1".into(),
            channel: "local".into(),
            created_at: "now".into(),
        };
        let report = evaluate(&device("ESP8285", DeviceType::Rx), &artifact, Some(0)).unwrap();
        assert!(!report.compatible);
        assert!(report.issues.iter().any(|i| i.code == "image_too_large"));
    }

    #[test]
    fn blocks_mock() {
        let mut d = device("ESP32", DeviceType::Tx);
        d.is_mock = true;
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(b"1234").unwrap();
        let artifact = FirmwareArtifactRecord {
            id: "a".into(),
            firmware_version_id: "v".into(),
            hardware_target_id: None,
            file_path: file.path().display().to_string(),
            sha256: "abc".into(),
            file_size: 4,
            verified: true,
            manufacturer: "ExpressLRS".into(),
            radio_system: "ExpressLRS".into(),
            version: "1".into(),
            channel: "local".into(),
            created_at: "now".into(),
        };
        let report = evaluate(&d, &artifact, Some(0)).unwrap();
        assert!(!report.compatible);
    }
}
