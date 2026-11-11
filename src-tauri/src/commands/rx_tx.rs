use crate::adapters::adapter_for;
use crate::device_detection::{self, DetectedDevice};
use crate::error::{AppError, AppResult};
use crate::firmware::compute_sha256;
use crate::state::AppState;
use serde_json::Value;
use std::fs;
use tauri::State;

fn find_device(state: &AppState, id: &str) -> AppResult<DetectedDevice> {
    let devices = state.devices.read();
    devices
        .iter()
        .find(|d| d.id == id)
        .cloned()
        .ok_or_else(|| AppError::DeviceNotFound(id.to_string()))
}

#[tauri::command]
pub fn list_rx_devices(state: State<'_, AppState>) -> Vec<DetectedDevice> {
    state
        .devices
        .read()
        .iter()
        .filter(|d| {
            use crate::device_detection::types::DeviceType;
            d.device_type == DeviceType::Rx
        })
        .cloned()
        .collect()
}

#[tauri::command]
pub fn list_tx_devices(state: State<'_, AppState>) -> Vec<DetectedDevice> {
    state
        .devices
        .read()
        .iter()
        .filter(|d| {
            use crate::device_detection::types::DeviceType;
            d.device_type == DeviceType::Tx
        })
        .cloned()
        .collect()
}

#[tauri::command]
pub fn get_device_detail(state: State<'_, AppState>, id: String) -> AppResult<DetectedDevice> {
    find_device(&state, &id)
}

#[tauri::command]
pub fn read_device_configuration(state: State<'_, AppState>, id: String) -> AppResult<Value> {
    let device = find_device(&state, &id)?;
    let adapter = adapter_for(&device).ok_or_else(|| {
        AppError::NotSupported(format!(
            "No adapter registered for device {} ({})",
            device.id,
            device
                .manufacturer
                .clone()
                .unwrap_or_else(|| "unknown".into())
        ))
    })?;
    adapter
        .read_configuration(&device)
        .map_err(crate::error::AppError::from)
}

#[tauri::command]
pub fn backup_device_configuration(state: State<'_, AppState>, id: String) -> AppResult<String> {
    let device = find_device(&state, &id)?;
    let adapter = adapter_for(&device).ok_or_else(|| {
        AppError::NotSupported(format!("No adapter registered for device {}", device.id))
    })?;
    let config = adapter
        .read_configuration(&device)
        .map_err(crate::error::AppError::from)?;
    let backup_dir = state.data_dir.join("backups");
    fs::create_dir_all(&backup_dir)?;
    let file_name = format!("{}_{}.json", device.id, chrono::Utc::now().format("%Y%m%d_%H%M%S"));
    let file_path = backup_dir.join(&file_name);
    let json = serde_json::to_vec_pretty(&config).map_err(|e| AppError::Message(e.to_string()))?;
    fs::write(&file_path, json)?;
    let (sha256, _) = compute_sha256(&file_path)?;

    let backup_id = state.db.insert_backup(
        Some(&device.id),
        "configuration",
        Some(file_path.to_string_lossy().as_ref()),
        Some(&sha256),
        device.model.as_deref(),
        device.firmware_version.as_deref(),
        Some("RX/TX configuration backup"),
    )?;

    state.db.record_operation(
        "config_backup",
        Some(&device.id),
        "success",
        &format!("Saved configuration backup {backup_id}"),
        None,
    )?;
    Ok(backup_id)
}

#[tauri::command]
pub fn reconnect_device(state: State<'_, AppState>, id: String) -> AppResult<DetectedDevice> {
    let mock = state.mock_mode_enabled();
    let scanned = device_detection::scan_devices(&state.ports, mock)?;
    for device in &scanned {
        let _ = state.db.upsert_device_snapshot(device);
    }
    *state.devices.write() = scanned;
    find_device(&state, &id)
}
