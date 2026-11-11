use crate::database::{Database, FirmwareArtifactFilter, FirmwareArtifactRecord, ImportFirmwareMeta};
use crate::error::{AppError, AppResult};
use crate::firmware::validation::{compute_sha256, extension_allowed};
use std::fs;
use std::path::{Path, PathBuf};

pub fn import_firmware_file(
    db: &Database,
    data_dir: &Path,
    source_path: &Path,
    meta: ImportFirmwareMeta,
) -> AppResult<FirmwareArtifactRecord> {
    if !source_path.exists() {
        return Err(AppError::Message(format!(
            "Source firmware file not found: {}",
            source_path.display()
        )));
    }
    if !extension_allowed(source_path) {
        return Err(AppError::Message(
            "Firmware import requires .bin, .elf, or .hex extension".into(),
        ));
    }

    let firmware_dir = data_dir.join("firmware");
    fs::create_dir_all(&firmware_dir)?;

    let ext = source_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("bin");
    let dest_name = format!("{}.{}", uuid::Uuid::new_v4(), ext);
    let dest_path = firmware_dir.join(&dest_name);
    fs::copy(source_path, &dest_path)?;

    let (sha256, size) = compute_sha256(&dest_path)?;
    let channel = meta.channel.as_deref().unwrap_or("local");

    let version_id = db.insert_firmware_version(
        &meta.manufacturer,
        &meta.radio_system,
        &meta.version,
        channel,
        meta.source.as_deref(),
        meta.release_notes.as_deref(),
    )?;

    let artifact_id = db.insert_firmware_artifact(
        &version_id,
        meta.hardware_target_id.as_deref(),
        dest_path.to_string_lossy().as_ref(),
        &sha256,
        size as i64,
        false,
    )?;

    db.get_firmware_artifact(&artifact_id)?
        .ok_or_else(|| AppError::Message("Failed to load imported artifact".into()))
}

pub fn list_artifacts(
    db: &Database,
    filter: FirmwareArtifactFilter,
) -> AppResult<Vec<FirmwareArtifactRecord>> {
    db.list_firmware_artifacts(&filter)
}

pub fn resolve_artifact_path(record: &FirmwareArtifactRecord) -> PathBuf {
    PathBuf::from(&record.file_path)
}
