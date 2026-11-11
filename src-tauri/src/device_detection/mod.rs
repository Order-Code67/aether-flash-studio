pub mod elrs_map;
pub mod mock;
pub mod profiles;
pub mod types;

pub use profiles::*;
pub use types::*;

use crate::error::AppResult;
use crate::serial::{list_com_ports, PortLockManager};
use crate::usb::{enumerate_usb_devices, stable_usb_id};

fn classify_usb(
    vid: Option<u16>,
    pid: Option<u16>,
    product: Option<&str>,
    manufacturer: Option<&str>,
) -> Classification {
    let product_l = product.unwrap_or("").to_lowercase();

    if let Some(hint) = elrs_map::lookup(vid, pid, product, manufacturer) {
        return Classification {
            manufacturer: Some(hint.manufacturer.to_string()),
            device_type: hint.device_type,
            model: Some(if product.unwrap_or("").is_empty() {
                hint.model.to_string()
            } else {
                product.unwrap().to_string()
            }),
            mcu: Some(hint.mcu.to_string()),
            radio_system: Some("ExpressLRS".into()),
            frequency_band: Some(hint.frequency_band.to_string()),
            flash_method: Some(hint.flash_method.to_string()),
            connection_mode: Some("USB CDC / UART".into()),
            bootloader_mode: hint.bootloader_likely,
            status: elrs_map::to_status(hint.bootloader_likely),
        };
    }

    if vid == Some(0x303a) {
        return Classification {
            manufacturer: Some("Espressif".into()),
            device_type: DeviceType::Generic,
            model: Some(product.unwrap_or("ESP32 USB Device").into()),
            mcu: Some("ESP32".into()),
            radio_system: None,
            frequency_band: None,
            flash_method: Some("ESP Bootloader / USB CDC".into()),
            connection_mode: Some("USB CDC".into()),
            bootloader_mode: product_l.contains("boot") || product_l.contains("serial"),
            status: DeviceStatus::Online,
        };
    }

    if vid == Some(0x10c4) && matches!(pid, Some(0xea60) | Some(0xea61) | Some(0xea63) | Some(0xea70))
    {
        return Classification {
            manufacturer: manufacturer
                .map(|s| s.to_string())
                .or(Some("Silicon Labs".into())),
            device_type: DeviceType::Generic,
            model: product.map(|s| s.to_string()),
            mcu: None,
            radio_system: None,
            frequency_band: None,
            flash_method: Some("UART / USB CDC".into()),
            connection_mode: Some("USB CDC".into()),
            bootloader_mode: false,
            status: DeviceStatus::Online,
        };
    }

    if vid == Some(0x1a86) {
        return Classification {
            manufacturer: manufacturer.map(|s| s.to_string()).or(Some("WCH".into())),
            device_type: DeviceType::Generic,
            model: product
                .map(|s| s.to_string())
                .or(Some("CH340 UART Bridge".into())),
            mcu: None,
            radio_system: None,
            frequency_band: None,
            flash_method: Some("UART".into()),
            connection_mode: Some("UART".into()),
            bootloader_mode: false,
            status: DeviceStatus::Unknown,
        };
    }

    if vid == Some(0x0403) {
        return Classification {
            manufacturer: manufacturer.map(|s| s.to_string()).or(Some("FTDI".into())),
            device_type: DeviceType::Generic,
            model: product.map(|s| s.to_string()),
            mcu: None,
            radio_system: None,
            frequency_band: None,
            flash_method: Some("UART".into()),
            connection_mode: Some("UART".into()),
            bootloader_mode: false,
            status: DeviceStatus::Unknown,
        };
    }

    Classification {
        manufacturer: manufacturer.map(|s| s.to_string()),
        device_type: DeviceType::Generic,
        model: product.map(|s| s.to_string()),
        mcu: None,
        radio_system: None,
        frequency_band: None,
        flash_method: None,
        connection_mode: Some("USB/UART".into()),
        bootloader_mode: false,
        status: DeviceStatus::Unknown,
    }
}

struct Classification {
    manufacturer: Option<String>,
    device_type: DeviceType,
    model: Option<String>,
    mcu: Option<String>,
    radio_system: Option<String>,
    frequency_band: Option<String>,
    flash_method: Option<String>,
    connection_mode: Option<String>,
    bootloader_mode: bool,
    status: DeviceStatus,
}

pub fn scan_devices(locks: &PortLockManager, mock_mode: bool) -> AppResult<Vec<DetectedDevice>> {
    let mut devices = Vec::new();
    let usb = enumerate_usb_devices(locks)?;
    let ports = list_com_ports(locks)?;

    for usb_dev in usb {
        // Prefer COM-backed USB CDC entries for operational device list.
        // Pure bus nodes without COM remain visible in USB inventory only.
        if usb_dev.com_port.is_none() && usb_dev.bus_path.is_some() {
            continue;
        }

        let class = classify_usb(
            usb_dev.vid,
            usb_dev.pid,
            usb_dev.product.as_deref(),
            usb_dev.manufacturer.as_deref(),
        );

        let name = if class.radio_system.as_deref() == Some("ExpressLRS") {
            class
                .model
                .clone()
                .unwrap_or_else(|| usb_dev.name.clone())
        } else if usb_dev.product.is_some() {
            usb_dev.name.clone()
        } else {
            "Generic Device".into()
        };

        devices.push(DetectedDevice {
            id: usb_dev.id,
            name,
            manufacturer: class.manufacturer,
            device_type: class.device_type,
            model: class.model,
            mcu: class.mcu,
            com_port: usb_dev.com_port,
            baud_rate: None,
            vid: usb_dev.vid,
            pid: usb_dev.pid,
            usb_serial: usb_dev.serial_number,
            radio_system: class.radio_system,
            frequency_band: class.frequency_band,
            firmware_version: None,
            bootloader_version: None,
            connection_mode: class.connection_mode,
            flash_method: class.flash_method,
            bootloader_mode: class.bootloader_mode,
            status: if class.bootloader_mode {
                DeviceStatus::Bootloader
            } else {
                class.status
            },
            is_mock: false,
            notes: None,
            profile_id: None,
        });
    }

    for port in ports {
        if port.port_type == "usb" {
            continue;
        }
        let id = stable_usb_id(None, None, None, &port.name);
        if devices
            .iter()
            .any(|d| d.com_port.as_deref() == Some(port.name.as_str()))
        {
            continue;
        }
        devices.push(DetectedDevice {
            id,
            name: "Generic Device".into(),
            manufacturer: port.manufacturer,
            device_type: DeviceType::Generic,
            model: port.product,
            mcu: None,
            com_port: Some(port.name),
            baud_rate: None,
            vid: port.vid,
            pid: port.pid,
            usb_serial: port.serial_number,
            radio_system: None,
            frequency_band: None,
            firmware_version: None,
            bootloader_version: None,
            connection_mode: Some("UART".into()),
            flash_method: None,
            bootloader_mode: false,
            status: DeviceStatus::Unknown,
            is_mock: false,
            notes: Some(
                "Identified by COM port presence only; model not assumed from port name".into(),
            ),
            profile_id: None,
        });
    }

    if mock_mode {
        devices.extend(mock::mock_devices());
    }

    devices.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(devices)
}

pub fn manual_connect(
    locks: &PortLockManager,
    request: ManualConnectRequest,
) -> AppResult<DetectedDevice> {
    locks.try_lock(&request.com_port)?;
    locks.unlock(&request.com_port);

    let ports = list_com_ports(locks)?;
    let matched = ports.iter().find(|p| p.name == request.com_port);

    let (vid, pid, serial, manufacturer, product) = matched
        .map(|p| {
            (
                p.vid,
                p.pid,
                p.serial_number.clone(),
                p.manufacturer.clone(),
                p.product.clone(),
            )
        })
        .unwrap_or((None, None, None, None, None));

    let class = classify_usb(vid, pid, product.as_deref(), manufacturer.as_deref());
    let id = stable_usb_id(vid, pid, serial.as_deref(), &request.com_port);

    Ok(DetectedDevice {
        id,
        name: product
            .clone()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "Generic Device".into()),
        manufacturer: class.manufacturer.or(manufacturer),
        device_type: class.device_type,
        model: class.model.or(product),
        mcu: class.mcu,
        com_port: Some(request.com_port),
        baud_rate: Some(request.baud_rate),
        vid,
        pid,
        usb_serial: serial,
        radio_system: class.radio_system,
        frequency_band: class.frequency_band,
        firmware_version: None,
        bootloader_version: None,
        connection_mode: Some(request.mode),
        flash_method: class.flash_method,
        bootloader_mode: class.bootloader_mode,
        status: DeviceStatus::Online,
        is_mock: false,
        notes: Some("Manually connected".into()),
        profile_id: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_not_guess_model_from_com_name() {
        let class = classify_usb(Some(0x1a86), Some(0x7523), None, None);
        assert_eq!(class.device_type, DeviceType::Generic);
    }

    #[test]
    fn mock_devices_are_marked() {
        assert!(mock::mock_devices().iter().all(|d| d.is_mock));
    }
}
