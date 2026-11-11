use crate::error::AppResult;
use crate::state::AppState;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoredConfiguration {
    pub id: String,
    pub device_id: Option<String>,
    pub profile_id: Option<String>,
    pub category: String,
    pub payload: Value,
    pub firmware_version: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveConfigurationRequest {
    pub device_id: Option<String>,
    pub profile_id: Option<String>,
    pub category: String,
    pub payload: Value,
    pub firmware_version: Option<String>,
}

pub fn save_configuration(
    state: &AppState,
    req: SaveConfigurationRequest,
) -> AppResult<StoredConfiguration> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let payload = serde_json::to_string(&req.payload)?;
    state.db.insert_device_configuration(
        &id,
        req.device_id.as_deref(),
        req.profile_id.as_deref(),
        &req.category,
        &payload,
        req.firmware_version.as_deref(),
        &now,
    )?;
    Ok(StoredConfiguration {
        id,
        device_id: req.device_id,
        profile_id: req.profile_id,
        category: req.category,
        payload: req.payload,
        firmware_version: req.firmware_version,
        created_at: now,
    })
}

pub fn list_configurations(
    state: &AppState,
    device_id: Option<String>,
) -> AppResult<Vec<StoredConfiguration>> {
    state.db.list_device_configurations(device_id.as_deref())
}

pub fn compare_configurations(a: &Value, b: &Value) -> Value {
    serde_json::json!({
        "equal": a == b,
        "left": a,
        "right": b,
    })
}
