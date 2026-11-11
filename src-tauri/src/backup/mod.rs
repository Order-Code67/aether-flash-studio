use crate::database::BackupRecord;
use crate::error::{AppError, AppResult};
use crate::firmware::compute_sha256;
use crate::state::AppState;
use chrono::Utc;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBackupRequest {
    pub device_id: Option<String>,
    pub backup_type: String,
    pub description: Option<String>,
    pub payload_json: String,
    pub model: Option<String>,
    pub firmware_version: Option<String>,
}

pub fn list_backups(state: &AppState) -> AppResult<Vec<BackupRecord>> {
    state.db.list_backups(200)
}

pub fn create_backup(state: &AppState, req: CreateBackupRequest) -> AppResult<BackupRecord> {
    let dir = state.data_dir.join("backups");
    fs::create_dir_all(&dir)?;
    let temp_name = format!("{}.json", uuid::Uuid::new_v4());
    let path = dir.join(&temp_name);
    fs::write(&path, req.payload_json.as_bytes())?;
    let (sha, _) = compute_sha256(&path)?;
    let id = state.db.insert_backup(
        req.device_id.as_deref(),
        &req.backup_type,
        Some(path.display().to_string().as_str()),
        Some(sha.as_str()),
        req.model.as_deref(),
        req.firmware_version.as_deref(),
        req.description.as_deref(),
    )?;
    // Rename to stable id-based filename when possible
    let final_path = dir.join(format!("{id}.json"));
    if final_path != path {
        let _ = fs::rename(&path, &final_path);
    }
    state.db.record_operation(
        "backup",
        req.device_id.as_deref(),
        "success",
        &format!("Created backup {id}"),
        None,
    )?;
    state
        .db
        .get_backup(&id)?
        .ok_or_else(|| AppError::Message("Backup insert failed".into()))
}

pub fn delete_backup(state: &AppState, id: &str) -> AppResult<()> {
    if let Some(record) = state.db.get_backup(id)? {
        if let Some(path) = record.file_path {
            let _ = fs::remove_file(path);
        }
    }
    state.db.delete_backup(id)?;
    Ok(())
}

pub fn compare_backups(state: &AppState, a: &str, b: &str) -> AppResult<serde_json::Value> {
    let left = state
        .db
        .get_backup(a)?
        .ok_or_else(|| AppError::Message("Backup A not found".into()))?;
    let right = state
        .db
        .get_backup(b)?
        .ok_or_else(|| AppError::Message("Backup B not found".into()))?;
    Ok(serde_json::json!({
        "shaEqual": left.sha256 == right.sha256,
        "typeA": left.backup_type,
        "typeB": right.backup_type,
        "modelA": left.model,
        "modelB": right.model,
        "createdA": left.created_at,
        "createdB": right.created_at,
    }))
}

pub fn export_backup(state: &AppState, id: &str, destination: &str) -> AppResult<String> {
    let record = state
        .db
        .get_backup(id)?
        .ok_or_else(|| AppError::Message("Backup not found".into()))?;
    let src = PathBuf::from(
        record
            .file_path
            .ok_or_else(|| AppError::Message("Backup has no file".into()))?,
    );
    let dest = PathBuf::from(destination);
    fs::copy(&src, &dest)?;
    Ok(dest.display().to_string())
}

pub fn import_backup_file(state: &AppState, path: &str) -> AppResult<BackupRecord> {
    let src = PathBuf::from(path);
    let payload = fs::read_to_string(&src)?;
    create_backup(
        state,
        CreateBackupRequest {
            device_id: None,
            backup_type: "imported".into(),
            description: Some(format!("Imported at {}", Utc::now().to_rfc3339())),
            payload_json: payload,
            model: None,
            firmware_version: None,
        },
    )
}
