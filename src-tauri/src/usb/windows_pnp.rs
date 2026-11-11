use crate::device_detection::types::UsbDeviceInfo;
use crate::error::AppResult;
use crate::usb::stable_usb_id;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PnpRow {
    #[serde(default, rename = "Name")]
    name: Option<String>,
    #[serde(default, rename = "Manufacturer")]
    manufacturer: Option<String>,
    #[serde(default, rename = "PNPDeviceID")]
    pnp_device_id: Option<String>,
    #[serde(default, rename = "Status")]
    status: Option<String>,
}

/// Enumerate USB PnP entities via CIM (Windows). Complements serialport CDC listing.
pub fn enumerate_pnp_usb() -> AppResult<Vec<UsbDeviceInfo>> {
    #[cfg(windows)]
    {
        enumerate_pnp_usb_windows()
    }
    #[cfg(not(windows))]
    {
        Ok(Vec::new())
    }
}

#[cfg(windows)]
fn enumerate_pnp_usb_windows() -> AppResult<Vec<UsbDeviceInfo>> {
    use std::process::Command;

    let script = r#"
$ErrorActionPreference = 'SilentlyContinue'
Get-CimInstance Win32_PnPEntity |
  Where-Object { $_.PNPDeviceID -like 'USB\\*' -or $_.PNPDeviceID -like 'USBSTOR\\*' } |
  Select-Object Name, Manufacturer, PNPDeviceID, Status |
  ConvertTo-Json -Compress
"#;

    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            script,
        ])
        .output()?;

    if !output.status.success() {
        tracing::warn!(
            category = "HARDWARE",
            "PnP USB enumeration failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return Ok(Vec::new());
    }

    let raw = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if raw.is_empty() || raw == "null" {
        return Ok(Vec::new());
    }

    let rows: Vec<PnpRow> = if raw.starts_with('[') {
        serde_json::from_str(&raw).unwrap_or_default()
    } else {
        serde_json::from_str::<PnpRow>(&raw)
            .map(|r| vec![r])
            .unwrap_or_default()
    };

    let mut devices = Vec::new();
    for row in rows {
        let pnp = row.pnp_device_id.unwrap_or_default();
        let (vid, pid, serial) = parse_pnp_id(&pnp);
        let name = row
            .name
            .clone()
            .unwrap_or_else(|| "USB Device".into());
        // Skip pure hubs noise lightly — still include composite/unknown USB nodes.
        if name.to_lowercase().contains("root hub") {
            continue;
        }
        let id = stable_usb_id(vid, pid, serial.as_deref(), &pnp);
        devices.push(UsbDeviceInfo {
            id,
            name,
            manufacturer: row.manufacturer,
            product: row.name,
            vid,
            pid,
            serial_number: serial,
            com_port: None,
            bus_path: Some(pnp),
            status: row.status,
        });
    }

    Ok(devices)
}

fn parse_pnp_id(pnp: &str) -> (Option<u16>, Option<u16>, Option<String>) {
    let upper = pnp.to_uppercase();
    let vid = extract_hex_after(&upper, "VID_");
    let pid = extract_hex_after(&upper, "PID_");
    let serial = upper
        .split('\\')
        .last()
        .map(|s| s.to_string())
        .filter(|s| !s.contains("VID_") && !s.is_empty());
    (vid, pid, serial)
}

fn extract_hex_after(hay: &str, marker: &str) -> Option<u16> {
    let idx = hay.find(marker)?;
    let start = idx + marker.len();
    let slice = hay.get(start..start + 4)?;
    u16::from_str_radix(slice, 16).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_pnp_vid_pid() {
        let (vid, pid, serial) =
            parse_pnp_id(r"USB\VID_10C4&PID_EA60\0001");
        assert_eq!(vid, Some(0x10c4));
        assert_eq!(pid, Some(0xea60));
        assert_eq!(serial.as_deref(), Some("0001"));
    }
}
