use crate::error::AppResult;
use crate::hardware::{self, HardwareServiceStatus};
use crate::state::AppState;
use serde::Serialize;
use tauri::State;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub data_dir: String,
    pub database_path: String,
    pub platform: String,
}

#[tauri::command]
pub fn get_app_info(state: State<'_, AppState>) -> AppResult<AppInfo> {
    Ok(AppInfo {
        name: "Aether Flash Studio".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        data_dir: state.data_dir.display().to_string(),
        database_path: state.db.path().display().to_string(),
        platform: std::env::consts::OS.into(),
    })
}

#[tauri::command]
pub fn get_hardware_status() -> HardwareServiceStatus {
    hardware::status()
}

#[tauri::command]
pub fn get_operation_history(
    state: State<'_, AppState>,
    limit: Option<i64>,
) -> AppResult<Vec<crate::database::OperationRecord>> {
    state.db.recent_operations(limit.unwrap_or(25))
}
