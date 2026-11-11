use crate::device_detection::DetectedDevice;
use crate::firmware::check_build_tools;
use crate::serial::list_com_ports;
use crate::state::AppState;
use crate::usb::enumerate_usb_devices;
use chrono::Utc;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticReport {
    pub id: String,
    pub created_at: String,
    pub summary: serde_json::Value,
}

pub fn generate(state: &AppState, devices: &[DetectedDevice]) -> DiagnosticReport {
    let id = uuid::Uuid::new_v4().to_string();
    let created_at = Utc::now().to_rfc3339();
    let ports = list_com_ports(&state.ports).unwrap_or_default();
    let usb = enumerate_usb_devices(&state.ports).unwrap_or_default();
    let tools = check_build_tools();
    let db_ok = state.db.get_setting("app_version").is_ok();
    let errors = state.last_errors.read().clone();

    let summary = json!({
        "appVersion": env!("CARGO_PKG_VERSION"),
        "platform": std::env::consts::OS,
        "dataDir": state.data_dir.display().to_string(),
        "databaseOk": db_ok,
        "mockDeviceMode": state.mock_mode_enabled(),
        "comPorts": ports.len(),
        "usbDevices": usb.len(),
        "detectedDevices": devices.len(),
        "lockedPorts": state.ports.locked_ports(),
        "buildTools": tools,
        "devices": devices.iter().map(|d| json!({
            "id": d.id,
            "name": d.name,
            "type": d.device_type.as_str(),
            "status": d.status.as_str(),
            "comPort": d.com_port,
            "vid": d.vid,
            "pid": d.pid,
            "isMock": d.is_mock,
            "radioSystem": d.radio_system,
        })).collect::<Vec<_>>(),
        "recentErrors": errors,
        "sanitized": true,
        "note": "Diagnostic report excludes passwords, keys and bind phrases."
    });

    let _ = state.db.record_operation(
        "diagnostics",
        None,
        "success",
        "Generated diagnostic report",
        None,
    );

    DiagnosticReport {
        id,
        created_at,
        summary,
    }
}
