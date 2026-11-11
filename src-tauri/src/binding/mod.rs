use crate::error::{AppError, AppResult};
use crate::security::{protect_secret, unprotect_secret};
use crate::state::AppState;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BindingProfile {
    pub id: String,
    pub name: String,
    pub device_id: Option<String>,
    pub bind_phrase_set: bool,
    pub uid_preview: Option<String>,
    pub notes: Option<String>,
    pub updated_at: String,
    pub storage: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveBindingRequest {
    pub id: Option<String>,
    pub name: String,
    pub device_id: Option<String>,
    pub bind_phrase: Option<String>,
    pub notes: Option<String>,
}

fn binding_dir(state: &AppState) -> std::path::PathBuf {
    state.data_dir.join("binding")
}

pub fn list_profiles(state: &AppState) -> AppResult<Vec<BindingProfile>> {
    let dir = binding_dir(state);
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.path().extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(entry.path())?;
        if let Ok(profile) = serde_json::from_str::<BindingProfile>(&raw) {
            out.push(profile);
        }
    }
    out.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(out)
}

pub fn save_profile(state: &AppState, req: SaveBindingRequest) -> AppResult<BindingProfile> {
    let dir = binding_dir(state);
    fs::create_dir_all(&dir)?;
    let id = req
        .id
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let phrase_set = req
        .bind_phrase
        .as_ref()
        .map(|p| !p.is_empty())
        .unwrap_or(false);
    if let Some(phrase) = req.bind_phrase.as_ref().filter(|p| !p.is_empty()) {
        let protected = protect_secret(phrase.as_bytes())?;
        let secret_path = dir.join(format!("{id}.dpapi"));
        fs::write(secret_path, protected)?;
        // Remove legacy hash secret if present
        let _ = fs::remove_file(dir.join(format!("{id}.secret")));
    }
    let profile = BindingProfile {
        id: id.clone(),
        name: req.name,
        device_id: req.device_id,
        bind_phrase_set: phrase_set || dir.join(format!("{id}.dpapi")).exists(),
        uid_preview: None,
        notes: req.notes,
        updated_at: Utc::now().to_rfc3339(),
        storage: "windows_dpapi".into(),
    };
    fs::write(
        dir.join(format!("{id}.json")),
        serde_json::to_string_pretty(&profile)?,
    )?;
    // Never log bind phrase.
    state.db.record_operation(
        "binding",
        profile.device_id.as_deref(),
        "success",
        &format!("Saved binding profile {} (DPAPI)", profile.name),
        None,
    )?;
    Ok(profile)
}

/// Reveals bind phrase only when explicitly requested by a dedicated command.
pub fn reveal_bind_phrase(state: &AppState, id: &str) -> AppResult<String> {
    let path = binding_dir(state).join(format!("{id}.dpapi"));
    if !path.exists() {
        return Err(AppError::Message(
            "No DPAPI-protected bind phrase for this profile".into(),
        ));
    }
    let protected = fs::read(path)?;
    let plain = unprotect_secret(&protected)?;
    String::from_utf8(plain).map_err(|_| AppError::Message("Invalid UTF-8 secret".into()))
}

pub fn compare_profiles(state: &AppState, a: &str, b: &str) -> AppResult<serde_json::Value> {
    let profiles = list_profiles(state)?;
    let left = profiles
        .iter()
        .find(|p| p.id == a)
        .ok_or_else(|| AppError::Message("Profile A not found".into()))?;
    let right = profiles
        .iter()
        .find(|p| p.id == b)
        .ok_or_else(|| AppError::Message("Profile B not found".into()))?;
    Ok(serde_json::json!({
        "nameMatch": left.name == right.name,
        "bothHavePhrase": left.bind_phrase_set && right.bind_phrase_set,
        "deviceA": left.device_id,
        "deviceB": right.device_id,
        "compatibleHint": left.bind_phrase_set && right.bind_phrase_set,
        "storage": "windows_dpapi",
        "note": "Actual phrase values are never returned by compare."
    }))
}
