use crate::device_detection::{DashboardSnapshot, DeviceStatus, DeviceType};
use crate::error::AppResult;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub fn get_dashboard(state: State<'_, AppState>) -> AppResult<DashboardSnapshot> {
    let devices = state.devices.read().clone();
    let mock_device_mode = state.mock_mode_enabled();
    let recent_operations = state.db.recent_operations(12)?;
    let last_firmware_used = state
        .db
        .get_setting("last_firmware_used")?
        .filter(|v| !v.is_empty());
    let errors = state.last_errors.read().clone();

    let connected_rx = devices
        .iter()
        .filter(|d| d.device_type == DeviceType::Rx && d.status != DeviceStatus::Offline)
        .count();
    let connected_tx = devices
        .iter()
        .filter(|d| d.device_type == DeviceType::Tx && d.status != DeviceStatus::Offline)
        .count();

    let usb_ok = crate::usb::enumerate_usb_devices(&state.ports).is_ok();
    let uart_ok = crate::serial::list_com_ports(&state.ports).is_ok();

    Ok(DashboardSnapshot {
        detected_devices: devices.len(),
        connected_rx,
        connected_tx,
        usb_ok,
        uart_ok,
        last_firmware_used,
        recent_operations,
        errors,
        firmware_updates_available: 0,
        build_tools_status: "Not configured (Phase 4)".into(),
        mock_device_mode,
        devices,
    })
}
