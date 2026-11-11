use crate::backup::{self, CreateBackupRequest};
use crate::binding::{self, SaveBindingRequest};
use crate::bootloader;
use crate::configuration::{self, SaveConfigurationRequest};
use crate::diagnostics;
use crate::error::{AppError, AppResult};
use crate::recovery;
use crate::serial_monitor::{crsf, OpenSerialRequest};
use crate::state::AppState;
use crate::unlock;
use serde_json::Value;
use tauri::{AppHandle, State};

fn device(state: &AppState, id: &str) -> AppResult<crate::device_detection::DetectedDevice> {
    state
        .devices
        .read()
        .iter()
        .find(|d| d.id == id)
        .cloned()
        .ok_or_else(|| AppError::DeviceNotFound(id.to_string()))
}

#[tauri::command]
pub fn inspect_bootloader(state: State<'_, AppState>, id: String) -> AppResult<bootloader::BootloaderStatus> {
    Ok(bootloader::inspect(&device(&state, &id)?))
}

#[tauri::command]
pub fn enter_bootloader_cmd(state: State<'_, AppState>, id: String) -> AppResult<bootloader::BootloaderStatus> {
    let d = device(&state, &id)?;
    let status = bootloader::enter_bootloader(&d)?;
    let _ = state.db.record_operation(
        "bootloader",
        Some(&id),
        "success",
        "Entered documented bootloader/download mode",
        None,
    );
    Ok(status)
}

#[tauri::command]
pub fn exit_bootloader_cmd(state: State<'_, AppState>, id: String) -> AppResult<bootloader::BootloaderStatus> {
    let d = device(&state, &id)?;
    bootloader::exit_bootloader(&d)
}

#[tauri::command]
pub fn diagnose_recovery(state: State<'_, AppState>, id: String) -> AppResult<recovery::RecoveryDiagnosis> {
    Ok(recovery::diagnose(&device(&state, &id)?))
}

#[tauri::command]
pub fn inspect_unlock(state: State<'_, AppState>, id: String) -> AppResult<unlock::UnlockReport> {
    Ok(unlock::inspect_unlock(&device(&state, &id)?))
}

#[tauri::command]
pub fn apply_unlock_cmd(
    state: State<'_, AppState>,
    id: String,
    capability_id: String,
) -> AppResult<String> {
    let d = device(&state, &id)?;
    unlock::apply_unlock(&d, &capability_id).map_err(AppError::Message)
}

#[tauri::command]
pub fn generate_diagnostic_report(state: State<'_, AppState>) -> diagnostics::DiagnosticReport {
    let devices = state.devices.read().clone();
    diagnostics::generate(&state, &devices)
}

#[tauri::command]
pub fn save_configuration_cmd(
    state: State<'_, AppState>,
    request: SaveConfigurationRequest,
) -> AppResult<configuration::StoredConfiguration> {
    configuration::save_configuration(&state, request)
}

#[tauri::command]
pub fn list_configurations_cmd(
    state: State<'_, AppState>,
    device_id: Option<String>,
) -> AppResult<Vec<configuration::StoredConfiguration>> {
    configuration::list_configurations(&state, device_id)
}

#[tauri::command]
pub fn compare_configurations_cmd(left: Value, right: Value) -> Value {
    configuration::compare_configurations(&left, &right)
}

#[tauri::command]
pub fn list_binding_profiles(state: State<'_, AppState>) -> AppResult<Vec<binding::BindingProfile>> {
    binding::list_profiles(&state)
}

#[tauri::command]
pub fn save_binding_profile(
    state: State<'_, AppState>,
    request: SaveBindingRequest,
) -> AppResult<binding::BindingProfile> {
    binding::save_profile(&state, request)
}

#[tauri::command]
pub fn compare_binding_profiles(
    state: State<'_, AppState>,
    a: String,
    b: String,
) -> AppResult<Value> {
    binding::compare_profiles(&state, &a, &b)
}

#[tauri::command]
pub fn list_backups_cmd(state: State<'_, AppState>) -> AppResult<Vec<crate::database::BackupRecord>> {
    backup::list_backups(&state)
}

#[tauri::command]
pub fn create_backup_cmd(
    state: State<'_, AppState>,
    request: CreateBackupRequest,
) -> AppResult<crate::database::BackupRecord> {
    backup::create_backup(&state, request)
}

#[tauri::command]
pub fn delete_backup_cmd(state: State<'_, AppState>, id: String) -> AppResult<()> {
    backup::delete_backup(&state, &id)
}

#[tauri::command]
pub fn compare_backups_cmd(
    state: State<'_, AppState>,
    a: String,
    b: String,
) -> AppResult<Value> {
    backup::compare_backups(&state, &a, &b)
}

#[tauri::command]
pub fn export_backup_cmd(
    state: State<'_, AppState>,
    id: String,
    destination: String,
) -> AppResult<String> {
    backup::export_backup(&state, &id, &destination)
}

#[tauri::command]
pub fn import_backup_cmd(state: State<'_, AppState>, path: String) -> AppResult<crate::database::BackupRecord> {
    backup::import_backup_file(&state, &path)
}

#[tauri::command]
pub fn open_serial_monitor(
    app: AppHandle,
    state: State<'_, AppState>,
    request: OpenSerialRequest,
) -> AppResult<()> {
    state.serial_hub.open(app, &state.ports, request)
}

#[tauri::command]
pub fn close_serial_monitor(state: State<'_, AppState>) -> AppResult<()> {
    state.serial_hub.close(&state.ports)
}

#[tauri::command]
pub fn send_serial_monitor(
    state: State<'_, AppState>,
    data: String,
    hex: bool,
) -> AppResult<()> {
    state.serial_hub.send(&data, hex)
}

#[tauri::command]
pub fn decode_crsf_preview(bytes_hex: String) -> AppResult<Vec<crsf::CrsfFrameView>> {
    let cleaned: String = bytes_hex.chars().filter(|c| !c.is_whitespace()).collect();
    if cleaned.len() % 2 != 0 {
        return Err(AppError::Message("HEX length must be even".into()));
    }
    let mut bytes = Vec::new();
    for i in (0..cleaned.len()).step_by(2) {
        bytes.push(
            u8::from_str_radix(&cleaned[i..i + 2], 16)
                .map_err(|_| AppError::Message("Invalid HEX".into()))?,
        );
    }
    Ok(crsf::decode_preview(&bytes))
}
