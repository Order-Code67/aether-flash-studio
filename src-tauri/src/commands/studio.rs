use crate::compatibility::{self, CompatibilityReport};
use crate::elrs_sources::{
    self, ElrsReleaseInfo, ElrsTagInfo, FetchSourceRequest, SourceDownloadRecord,
};
use crate::error::{AppError, AppResult};
use crate::firmware::{
    check_build_tools, import_firmware_file, start_firmware_build, BuildToolStatus,
    FirmwareBuildRequest,
};
use crate::state::AppState;
use crate::workspace::{self, FileContent, WorkspaceNode};
use crate::database::ImportFirmwareMeta;
use serde::Deserialize;
use std::path::PathBuf;
use tauri::State;

#[tauri::command]
pub fn list_elrs_releases() -> AppResult<Vec<ElrsReleaseInfo>> {
    elrs_sources::list_official_releases()
}

#[tauri::command]
pub fn list_elrs_tags() -> AppResult<Vec<ElrsTagInfo>> {
    elrs_sources::list_official_tags()
}

#[tauri::command]
pub fn fetch_elrs_source(
    state: State<'_, AppState>,
    request: FetchSourceRequest,
) -> AppResult<SourceDownloadRecord> {
    elrs_sources::fetch_official_source(&state.db, &state.data_dir, request)
}

#[tauri::command]
pub fn list_elrs_local_sources(
    state: State<'_, AppState>,
) -> AppResult<Vec<SourceDownloadRecord>> {
    elrs_sources::list_local_sources(&state.db)
}

#[tauri::command]
pub fn verify_elrs_local_source(path: String) -> AppResult<bool> {
    elrs_sources::verify_local_source(PathBuf::from(path).as_path())
}

#[tauri::command]
pub fn workspace_list_tree(
    root: String,
    max_entries: Option<usize>,
) -> AppResult<Vec<WorkspaceNode>> {
    workspace::list_tree(PathBuf::from(root).as_path(), max_entries.unwrap_or(4000))
}

#[tauri::command]
pub fn workspace_read_file(root: String, relative: String) -> AppResult<FileContent> {
    workspace::read_file(PathBuf::from(root).as_path(), &relative)
}

#[tauri::command]
pub fn workspace_write_file(root: String, relative: String, content: String) -> AppResult<()> {
    workspace::write_file(PathBuf::from(root).as_path(), &relative, &content)
}

#[tauri::command]
pub fn workspace_revert_file(root: String, relative: String) -> AppResult<()> {
    workspace::revert_file(PathBuf::from(root).as_path(), &relative)
}

#[tauri::command]
pub fn workspace_search(root: String, query: String) -> AppResult<Vec<String>> {
    workspace::search_project(PathBuf::from(root).as_path(), &query, 200)
}

#[tauri::command]
pub fn workspace_diff(root: String, relative: String) -> AppResult<String> {
    workspace::diff_against_backup(PathBuf::from(root).as_path(), &relative)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomBuildRequest {
    pub manufacturer: String,
    pub device_type: String,
    pub platform: String,
    pub model: String,
    pub elrs_version: String,
    pub source_path: String,
    pub target: String,
    pub config: Option<serde_json::Value>,
}

#[tauri::command]
pub fn run_custom_firmware_build(
    state: State<'_, AppState>,
    request: CustomBuildRequest,
) -> AppResult<serde_json::Value> {
    if request.manufacturer != "ExpressLRS" {
        return Err(AppError::Message(
            "Custom builder currently supports ExpressLRS only".into(),
        ));
    }
    let root = PathBuf::from(&request.source_path);
    if !elrs_sources::verify_local_source(&root)? {
        return Err(AppError::Message(
            "Source path is not a verified official ExpressLRS checkout".into(),
        ));
    }

    let tools = check_build_tools();
    if !tools.platformio.available {
        return Err(AppError::Message(
            "PlatformIO (pio) is required for custom firmware builds".into(),
        ));
    }

    // Compatibility of selected platform labels (ESP32 / ESP8285)
    let platform_ok = matches!(
        request.platform.to_uppercase().as_str(),
        "ESP32" | "ESP8285" | "ESP8266"
    );
    if !platform_ok {
        return Err(AppError::Message(format!(
            "Unsupported platform '{}'",
            request.platform
        )));
    }

    let build_id = start_firmware_build(
        &state.db,
        FirmwareBuildRequest {
            source_path: Some(request.source_path.clone()),
            source_repo: Some(elrs_sources::OFFICIAL_REPO.into()),
            branch: Some(request.elrs_version.clone()),
            target: Some(request.target.clone()),
            config: request.config.clone().or_else(|| {
                Some(serde_json::json!({
                    "manufacturer": request.manufacturer,
                    "deviceType": request.device_type,
                    "platform": request.platform,
                    "model": request.model,
                }))
            }),
        },
    )?;

    // Attempt to import newest .bin under .pio/build if present after build record
    let maybe_bin = find_newest_bin(&root.join(".pio").join("build"));
    let mut imported = None;
    if let Some(bin) = maybe_bin {
        let art = import_firmware_file(
            &state.db,
            &state.data_dir,
            &bin,
            ImportFirmwareMeta {
                manufacturer: "ExpressLRS".into(),
                radio_system: "ExpressLRS".into(),
                version: request.elrs_version.clone(),
                channel: Some("custom_build".into()),
                source: Some(format!("local_build:{}", request.target)),
                release_notes: Some(format!("Built for {}", request.model)),
                hardware_target_id: Some(request.target.clone()),
            },
        )?;
        imported = Some(art.id);
    }

    Ok(serde_json::json!({
        "buildId": build_id,
        "tools": tools,
        "importedArtifactId": imported,
        "note": if imported.is_some() {
            "Build finished and artifact imported"
        } else {
            "Build recorded; import skipped if PlatformIO artifact not found yet"
        }
    }))
}

fn find_newest_bin(dir: &std::path::Path) -> Option<PathBuf> {
    if !dir.exists() {
        return None;
    }
    let mut best: Option<(std::time::SystemTime, PathBuf)> = None;
    for entry in walkdir::WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("bin") {
            if let Ok(meta) = path.metadata() {
                if let Ok(modified) = meta.modified() {
                    if best.as_ref().map(|(t, _)| modified > *t).unwrap_or(true) {
                        best = Some((modified, path.to_path_buf()));
                    }
                }
            }
        }
    }
    best.map(|(_, p)| p)
}

#[tauri::command]
pub fn evaluate_compatibility(
    state: State<'_, AppState>,
    device_id: String,
    artifact_id: String,
) -> AppResult<CompatibilityReport> {
    let device = state
        .devices
        .read()
        .iter()
        .find(|d| d.id == device_id)
        .cloned()
        .ok_or_else(|| AppError::DeviceNotFound(device_id))?;
    let artifact = state
        .db
        .get_firmware_artifact(&artifact_id)?
        .ok_or_else(|| AppError::Message("Artifact not found".into()))?;
    compatibility::evaluate(&device, &artifact, Some(0))
}

#[tauri::command]
pub fn get_flash_step_logs(
    state: State<'_, AppState>,
    session_id: String,
) -> AppResult<Vec<serde_json::Value>> {
    state.db.list_flash_step_logs(&session_id)
}

#[tauri::command]
pub fn reveal_bind_phrase_cmd(state: State<'_, AppState>, id: String) -> AppResult<String> {
    // Explicit user action only — caller must not log the value.
    crate::binding::reveal_bind_phrase(&state, &id)
}

#[tauri::command]
pub fn check_build_tools_studio() -> BuildToolStatus {
    check_build_tools()
}
