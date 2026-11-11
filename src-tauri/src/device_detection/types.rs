use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceType {
    Rx,
    Tx,
    Generic,
    Unknown,
}

impl DeviceType {
    pub fn as_str(self) -> &'static str {
        match self {
            DeviceType::Rx => "rx",
            DeviceType::Tx => "tx",
            DeviceType::Generic => "generic",
            DeviceType::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceStatus {
    Online,
    Offline,
    Bootloader,
    Updating,
    Recovery,
    Error,
    Unknown,
}

impl DeviceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            DeviceStatus::Online => "online",
            DeviceStatus::Offline => "offline",
            DeviceStatus::Bootloader => "bootloader",
            DeviceStatus::Updating => "updating",
            DeviceStatus::Recovery => "recovery",
            DeviceStatus::Error => "error",
            DeviceStatus::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectedDevice {
    pub id: String,
    pub name: String,
    pub manufacturer: Option<String>,
    pub device_type: DeviceType,
    pub model: Option<String>,
    pub mcu: Option<String>,
    pub com_port: Option<String>,
    pub baud_rate: Option<u32>,
    pub vid: Option<u16>,
    pub pid: Option<u16>,
    pub usb_serial: Option<String>,
    pub radio_system: Option<String>,
    pub frequency_band: Option<String>,
    pub firmware_version: Option<String>,
    pub bootloader_version: Option<String>,
    pub connection_mode: Option<String>,
    pub flash_method: Option<String>,
    pub bootloader_mode: bool,
    pub status: DeviceStatus,
    pub is_mock: bool,
    pub notes: Option<String>,
    pub profile_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsbDeviceInfo {
    pub id: String,
    pub name: String,
    pub manufacturer: Option<String>,
    pub product: Option<String>,
    pub vid: Option<u16>,
    pub pid: Option<u16>,
    pub serial_number: Option<String>,
    pub com_port: Option<String>,
    pub bus_path: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComPortInfo {
    pub name: String,
    pub port_type: String,
    pub vid: Option<u16>,
    pub pid: Option<u16>,
    pub serial_number: Option<String>,
    pub manufacturer: Option<String>,
    pub product: Option<String>,
    pub locked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManualConnectRequest {
    pub com_port: String,
    pub baud_rate: u32,
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardSnapshot {
    pub detected_devices: usize,
    pub connected_rx: usize,
    pub connected_tx: usize,
    pub usb_ok: bool,
    pub uart_ok: bool,
    pub last_firmware_used: Option<String>,
    pub recent_operations: Vec<crate::database::OperationRecord>,
    pub errors: Vec<String>,
    pub firmware_updates_available: usize,
    pub build_tools_status: String,
    pub mock_device_mode: bool,
    pub devices: Vec<DetectedDevice>,
}
