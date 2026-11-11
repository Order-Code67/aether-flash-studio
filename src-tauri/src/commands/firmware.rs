use crate::database::{FirmwareArtifactFilter, FirmwareArtifactRecord, ImportFirmwareMeta};
use crate::error::AppResult;
use crate::firmware::{
    check_build_tools, import_firmware_file, list_artifacts, start_firmware_build,
    validate_record, FirmwareBuildRequest, ValidationResult,
};
use crate::state::AppState;
use std::path::PathBuf;
use tauri::State;

#[tauri::command]
pub fn list_firmware_artifacts(
    state: State<'_, AppState>,
    filter: Option<FirmwareArtifactFilter>,
) -> AppResult<Vec<FirmwareArtifactRecord>> {
    list_artifacts(&state.db, filter.unwrap_or(FirmwareArtifactFilter {
        manufacturer: None,
        radio_system: None,
        version: None,
        verified_only: None,
        limit: Some(100),
    }))
}

#[tauri::command]
pub fn import_firmware_file_cmd(
    state: State<'_, AppState>,
    path: String,
    meta: ImportFirmwareMeta,
) -> AppResult<FirmwareArtifactRecord> {
    let record = import_firmware_file(&state.db, &state.data_dir, PathBuf::from(path).as_path(), meta)?;
    state.db.record_operation(
        "firmware_import",
        None,
        "success",
        &format!("Imported firmware artifact {}", record.id),
        None,
    )?;
    Ok(record)
}

#[tauri::command]
pub fn validate_firmware_artifact(
    state: State<'_, AppState>,
    id: String,
) -> AppResult<ValidationResult> {
    let record = state
        .db
        .get_firmware_artifact(&id)?
        .ok_or_else(|| crate::error::AppError::Message("Artifact not found".into()))?;
    let result = validate_record(&record)?;
    if result.valid {
        state.db.set_firmware_artifact_verified(&id, true)?;
    }
    Ok(result)
}

#[tauri::command]
pub fn check_build_tools_cmd() -> crate::firmware::BuildToolStatus {
    check_build_tools()
}

#[tauri::command]
pub fn start_firmware_build_cmd(
    state: State<'_, AppState>,
    request: FirmwareBuildRequest,
) -> AppResult<String> {
    start_firmware_build(&state.db, request)
}
