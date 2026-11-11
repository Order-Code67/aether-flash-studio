use crate::device_detection;
use crate::state::AppState;
use crate::usb::fingerprint_ports;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct DevicesChangedEvent {
    reason: String,
    device_count: usize,
    fingerprint: String,
}

pub fn start(app: &AppHandle) {
    let app = app.clone();
    tauri::async_runtime::spawn(async move {
        let mut last_fp = String::new();
        loop {
            tokio::time::sleep(Duration::from_secs(2)).await;
            let Some(state) = app.try_state::<AppState>() else {
                continue;
            };
            let fp = fingerprint_ports(&state.ports);
            if fp == last_fp {
                continue;
            }
            let first = last_fp.is_empty();
            last_fp = fp.clone();
            if first {
                // Establish baseline without emitting noise on startup.
                continue;
            }
            let mock = state.mock_mode_enabled();
            match device_detection::scan_devices(&state.ports, mock) {
                Ok(mut devices) => {
                    for device in &mut devices {
                        if let Ok(Some(profile)) = state.db.find_profile_for_device(&device.id) {
                            device.profile_id = Some(profile.id);
                        }
                        let _ = state.db.upsert_device_snapshot(device);
                    }
                    let count = devices.len();
                    *state.devices.write() = devices;
                    let _ = app.emit(
                        "devices-changed",
                        DevicesChangedEvent {
                            reason: "hotplug".into(),
                            device_count: count,
                            fingerprint: fp,
                        },
                    );
                    tracing::info!(category = "HARDWARE", count, "Hot-plug refresh applied");
                }
                Err(err) => {
                    state.push_error(err.to_string());
                    tracing::warn!(category = "HARDWARE", error = %err, "Hot-plug scan failed");
                }
            }
        }
    });
}
