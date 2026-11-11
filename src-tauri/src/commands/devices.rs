use crate::device_detection::{self, DetectedDevice, ManualConnectRequest};
use crate::error::AppResult;
use crate::serial::list_com_ports;
use crate::state::AppState;
use crate::usb::enumerate_usb_devices;
use tauri::State;

#[tauri::command]
pub fn list_com_ports_cmd(
    state: State<'_, AppState>,
) -> AppResult<Vec<crate::device_detection::ComPortInfo>> {
    list_com_ports(&state.ports)
}

#[tauri::command]
pub fn list_usb_devices_cmd(
    state: State<'_, AppState>,
) -> AppResult<Vec<crate::device_detection::UsbDeviceInfo>> {
    enumerate_usb_devices(&state.ports)
}

#[tauri::command]
pub fn scan_devices(state: State<'_, AppState>) -> AppResult<Vec<DetectedDevice>> {
    let mock = state.mock_mode_enabled();
    match device_detection::scan_devices(&state.ports, mock) {
        Ok(mut devices) => {
            for device in &mut devices {
                if let Ok(Some(profile)) = state.db.find_profile_for_device(&device.id) {
                    device.profile_id = Some(profile.id);
                }
                if let Err(err) = state.db.upsert_device_snapshot(device) {
                    tracing::warn!(category = "WARNING", error = %err, "Failed to persist device snapshot");
                }
            }
            state.db.record_operation(
                "device_scan",
                None,
                "success",
                &format!("Detected {} device(s)", devices.len()),
                None,
            )?;
            *state.devices.write() = devices.clone();
            tracing::info!(
                category = "HARDWARE",
                count = devices.len(),
                mock,
                "Device scan completed"
            );
            Ok(devices)
        }
        Err(err) => {
            state.push_error(err.to_string());
            let _ = state.db.record_operation(
                "device_scan",
                None,
                "error",
                "Device scan failed",
                Some(&err.to_string()),
            );
            Err(err)
        }
    }
}

#[tauri::command]
pub fn get_cached_devices(state: State<'_, AppState>) -> Vec<DetectedDevice> {
    state.devices.read().clone()
}

#[tauri::command]
pub fn manual_connect_device(
    state: State<'_, AppState>,
    request: ManualConnectRequest,
) -> AppResult<DetectedDevice> {
    match device_detection::manual_connect(&state.ports, request) {
        Ok(device) => {
            state.db.upsert_device_snapshot(&device)?;
            state.db.record_operation(
                "manual_connect",
                Some(&device.id),
                "success",
                &format!(
                    "Manual connect on {}",
                    device.com_port.clone().unwrap_or_default()
                ),
                None,
            )?;
            {
                let mut devices = state.devices.write();
                if let Some(existing) = devices.iter_mut().find(|d| d.id == device.id) {
                    *existing = device.clone();
                } else {
                    devices.push(device.clone());
                }
            }
            Ok(device)
        }
        Err(err) => {
            state.push_error(err.to_string());
            let _ = state.db.record_operation(
                "manual_connect",
                None,
                "error",
                "Manual connect failed",
                Some(&err.to_string()),
            );
            Err(err)
        }
    }
}

#[tauri::command]
pub fn get_locked_ports(state: State<'_, AppState>) -> Vec<String> {
    state.ports.locked_ports()
}
