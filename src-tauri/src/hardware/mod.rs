//! Hardware service façade.
//! Flash adapters and manufacturer plugins will plug in here in later phases.

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HardwareServiceStatus {
    pub usb_enumeration: bool,
    pub serial_enumeration: bool,
    pub port_locking: bool,
    pub flashing_engine: bool,
    pub mock_device_mode_available: bool,
}

pub fn status() -> HardwareServiceStatus {
    HardwareServiceStatus {
        usb_enumeration: true,
        serial_enumeration: true,
        port_locking: true,
        flashing_engine: true,
        mock_device_mode_available: true,
    }
}
