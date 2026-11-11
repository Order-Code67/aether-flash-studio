use crate::device_detection::types::ComPortInfo;
use crate::error::{AppError, AppResult};
use parking_lot::Mutex;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct PortLockManager {
    locked: Arc<Mutex<HashSet<String>>>,
}

impl PortLockManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_locked(&self, port: &str) -> bool {
        self.locked.lock().contains(port)
    }

    pub fn try_lock(&self, port: &str) -> AppResult<()> {
        let mut guard = self.locked.lock();
        if guard.contains(port) {
            return Err(AppError::PortLocked(port.to_string()));
        }
        guard.insert(port.to_string());
        tracing::info!(category = "HARDWARE", port, "Serial port locked");
        Ok(())
    }

    pub fn unlock(&self, port: &str) {
        self.locked.lock().remove(port);
        tracing::info!(category = "HARDWARE", port, "Serial port unlocked");
    }

    pub fn locked_ports(&self) -> Vec<String> {
        self.locked.lock().iter().cloned().collect()
    }
}

pub fn list_com_ports(locks: &PortLockManager) -> AppResult<Vec<ComPortInfo>> {
    let ports = serialport::available_ports().map_err(AppError::from)?;
    let mut out = Vec::new();

    for port in ports {
        let mut info = ComPortInfo {
            name: port.port_name.clone(),
            port_type: "unknown".into(),
            vid: None,
            pid: None,
            serial_number: None,
            manufacturer: None,
            product: None,
            locked: locks.is_locked(&port.port_name),
        };

        match port.port_type {
            serialport::SerialPortType::UsbPort(usb) => {
                info.port_type = "usb".into();
                info.vid = Some(usb.vid);
                info.pid = Some(usb.pid);
                info.serial_number = usb.serial_number;
                info.manufacturer = usb.manufacturer;
                info.product = usb.product;
            }
            serialport::SerialPortType::PciPort => info.port_type = "pci".into(),
            serialport::SerialPortType::BluetoothPort => info.port_type = "bluetooth".into(),
            serialport::SerialPortType::Unknown => info.port_type = "unknown".into(),
        }

        out.push(info);
    }

    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn port_lock_prevents_double_lock() {
        let manager = PortLockManager::new();
        manager.try_lock("COM3").unwrap();
        assert!(manager.try_lock("COM3").is_err());
        manager.unlock("COM3");
        manager.try_lock("COM3").unwrap();
    }
}
