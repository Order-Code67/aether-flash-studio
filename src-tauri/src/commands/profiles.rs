use crate::device_detection::{DeviceProfile, UpsertDeviceProfile};
use crate::error::AppResult;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub fn list_device_profiles(state: State<'_, AppState>) -> AppResult<Vec<DeviceProfile>> {
    state.db.list_device_profiles()
}

#[tauri::command]
pub fn upsert_device_profile(
    state: State<'_, AppState>,
    profile: UpsertDeviceProfile,
) -> AppResult<DeviceProfile> {
    let saved = state.db.upsert_device_profile(profile)?;
    state.db.record_operation(
        "device_profile",
        saved.device_id.as_deref(),
        "success",
        &format!("Saved profile {}", saved.name),
        None,
    )?;
    Ok(saved)
}

#[tauri::command]
pub fn delete_device_profile(state: State<'_, AppState>, id: String) -> AppResult<()> {
    state.db.delete_device_profile(&id)?;
    state.db.record_operation(
        "device_profile",
        None,
        "success",
        &format!("Deleted profile {id}"),
        None,
    )?;
    Ok(())
}
