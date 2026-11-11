use crate::error::AppResult;
use crate::state::AppState;
use serde::Serialize;
use tauri::State;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub mock_device_mode: bool,
    pub uart_default_baud: u32,
    pub last_firmware_used: Option<String>,
    pub locale: String,
}

fn normalize_locale(value: Option<String>) -> String {
    match value.as_deref() {
        Some("en") | Some("en-US") | Some("en_US") => "en".into(),
        _ => "ru".into(),
    }
}

#[tauri::command]
pub fn get_settings(state: State<'_, AppState>) -> AppResult<AppSettings> {
    let mock = state.mock_mode_enabled();
    let baud = state
        .db
        .get_setting("uart_default_baud")?
        .and_then(|v| v.parse().ok())
        .unwrap_or(460800);
    let last = state
        .db
        .get_setting("last_firmware_used")?
        .filter(|v| !v.is_empty());
    let locale = normalize_locale(state.db.get_setting("ui_locale")?);
    Ok(AppSettings {
        mock_device_mode: mock,
        uart_default_baud: baud,
        last_firmware_used: last,
        locale,
    })
}

#[tauri::command]
pub fn set_mock_device_mode(state: State<'_, AppState>, enabled: bool) -> AppResult<AppSettings> {
    state
        .db
        .set_setting("mock_device_mode", if enabled { "true" } else { "false" })?;
    state.db.record_operation(
        "settings",
        None,
        "success",
        if enabled {
            "Mock Device Mode enabled"
        } else {
            "Mock Device Mode disabled"
        },
        None,
    )?;
    tracing::info!(
        category = "INFO",
        enabled,
        "Mock Device Mode setting changed"
    );
    get_settings(state)
}

#[tauri::command]
pub fn set_ui_locale(state: State<'_, AppState>, locale: String) -> AppResult<AppSettings> {
    let normalized = normalize_locale(Some(locale));
    state.db.set_setting("ui_locale", &normalized)?;
    state.db.record_operation(
        "settings",
        None,
        "success",
        &format!("UI locale set to {normalized}"),
        None,
    )?;
    get_settings(state)
}
