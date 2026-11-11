pub mod windows_pnp;

use crate::device_detection::types::UsbDeviceInfo;
use crate::error::AppResult;
use crate::serial::{list_com_ports, PortLockManager};
use sha2::{Digest, Sha256};
use std::collections::HashSet;

/// USB enumeration: serial/USB-CDC ports + Windows PnP USB entities.
pub fn enumerate_usb_devices(locks: &PortLockManager) -> AppResult<Vec<UsbDeviceInfo>> {
    let mut devices = Vec::new();
    let mut seen = HashSet::new();

    let ports = list_com_ports(locks)?;
    for port in ports.into_iter().filter(|p| p.port_type == "usb") {
        let id = stable_usb_id(port.vid, port.pid, port.serial_number.as_deref(), &port.name);
        seen.insert(format!(
            "{:04x}:{:04x}:{}",
            port.vid.unwrap_or(0),
            port.pid.unwrap_or(0),
            port.serial_number.clone().unwrap_or_default()
        ));
        devices.push(UsbDeviceInfo {
            id,
            name: port
                .product
                .clone()
                .unwrap_or_else(|| format!("USB Device ({})", port.name)),
            manufacturer: port.manufacturer,
            product: port.product,
            vid: port.vid,
            pid: port.pid,
            serial_number: port.serial_number,
            com_port: Some(port.name),
            bus_path: None,
            status: Some("OK".into()),
        });
    }

    match windows_pnp::enumerate_pnp_usb() {
        Ok(pnp) => {
            for device in pnp {
                let key = format!(
                    "{:04x}:{:04x}:{}",
                    device.vid.unwrap_or(0),
                    device.pid.unwrap_or(0),
                    device.serial_number.clone().unwrap_or_default()
                );
                if seen.contains(&key) {
                    // Merge bus path onto existing CDC entry when possible.
                    if let Some(existing) = devices.iter_mut().find(|d| {
                        d.vid == device.vid
                            && d.pid == device.pid
                            && d.serial_number == device.serial_number
                    }) {
                        if existing.bus_path.is_none() {
                            existing.bus_path = device.bus_path;
                        }
                    }
                    continue;
                }
                seen.insert(key);
                devices.push(device);
            }
        }
        Err(err) => {
            tracing::warn!(category = "HARDWARE", error = %err, "PnP USB scan failed");
        }
    }

    devices.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(devices)
}

pub fn stable_usb_id(
    vid: Option<u16>,
    pid: Option<u16>,
    serial: Option<&str>,
    fallback: &str,
) -> String {
    let raw = format!(
        "{:04x}:{:04x}:{}:{}",
        vid.unwrap_or(0),
        pid.unwrap_or(0),
        serial.unwrap_or(""),
        fallback
    );
    let digest = Sha256::digest(raw.as_bytes());
    format!("usb-{}", hex::encode(&digest[..8]))
}

pub fn fingerprint_ports(locks: &PortLockManager) -> String {
    let ports = list_com_ports(locks).unwrap_or_default();
    let mut parts: Vec<String> = ports
        .into_iter()
        .map(|p| {
            format!(
                "{}:{:04x}:{:04x}:{}",
                p.name,
                p.vid.unwrap_or(0),
                p.pid.unwrap_or(0),
                p.serial_number.unwrap_or_default()
            )
        })
        .collect();
    parts.sort();
    parts.join("|")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_id_is_deterministic() {
        let a = stable_usb_id(Some(0x10c4), Some(0xea60), Some("ABC"), "COM5");
        let b = stable_usb_id(Some(0x10c4), Some(0xea60), Some("ABC"), "COM5");
        assert_eq!(a, b);
    }
}
