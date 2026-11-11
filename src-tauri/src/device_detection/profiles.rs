use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceProfile {
    pub id: String,
    pub device_id: Option<String>,
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub notes: Option<String>,
    pub preferred_com_port: Option<String>,
    pub preferred_baud: Option<u32>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpsertDeviceProfile {
    pub id: Option<String>,
    pub device_id: Option<String>,
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub notes: Option<String>,
    pub preferred_com_port: Option<String>,
    pub preferred_baud: Option<u32>,
}
