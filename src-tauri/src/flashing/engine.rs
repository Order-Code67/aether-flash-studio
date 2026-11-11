use super::adapters_uart::{esptool_chip_id, esptool_write_flash, resolve_esptool};
use crate::compatibility;
use crate::database::{Database, FlashSessionRecord};
use crate::device_detection::DetectedDevice;
use crate::error::{AppError, AppResult};
use crate::firmware::{validate_artifact_file, validate_record};
use crate::serial::{list_com_ports, PortLockManager};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlashProgressEvent {
    pub session_id: String,
    pub step: String,
    pub progress: f64,
    pub message: String,
    pub terminal: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareFlashRequest {
    pub device_id: String,
    pub artifact_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartFlashRequest {
    pub session_id: String,
    pub confirm: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlashSessionView {
    pub session: FlashSessionRecord,
    pub steps: Vec<String>,
}

const PIPELINE_STEPS: &[&str] = &[
    "initialized",
    "precheck_device",
    "compatibility",
    "validated_artifact",
    "port_checked",
    "esptool_ready",
    "connecting",
    "flashing",
    "post_write_probe",
    "port_rescan",
    "completed_unverified",
    "completed_verified",
    "failed",
];

fn emit_progress(app: &AppHandle, event: FlashProgressEvent) {
    let _ = app.emit("flash-progress", &event);
}

fn log_step(db: &Database, session_id: &str, step: &str, status: &str, message: &str) {
    let _ = db.insert_flash_step_log(session_id, step, status, Some(message));
}

pub fn prepare_flash_session(
    db: &Database,
    ports: &PortLockManager,
    device: &DetectedDevice,
    artifact_id: &str,
) -> AppResult<FlashSessionRecord> {
    if device.is_mock {
        return Err(AppError::NotSupported(
            "Mock devices cannot be used for flash sessions".into(),
        ));
    }

    let artifact = db
        .get_firmware_artifact(artifact_id)?
        .ok_or_else(|| AppError::Message("Firmware artifact not found".into()))?;

    let validation = validate_record(&artifact)?;
    if !validation.valid {
        return Err(AppError::Message(format!(
            "Artifact validation failed: {}",
            validation.messages.join("; ")
        )));
    }

    let report = compatibility::evaluate(device, &artifact, Some(0))?;
    if !report.compatible {
        let reasons: Vec<String> = report
            .issues
            .iter()
            .filter(|i| i.severity == "block")
            .map(|i| i.message.clone())
            .collect();
        return Err(AppError::Message(format!(
            "Compatibility blocked flash: {}",
            reasons.join("; ")
        )));
    }

    if let Some(port) = device.com_port.as_deref() {
        if ports.is_locked(port) {
            return Err(AppError::PortLocked(port.to_string()));
        }
    } else {
        return Err(AppError::Message(
            "Device has no COM port — connect the device before flashing".into(),
        ));
    }

    if resolve_esptool().is_none() {
        return Err(AppError::Message(
            "esptool is not installed or not on PATH — install esptool.py before flashing".into(),
        ));
    }

    let session_id = uuid::Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let record = FlashSessionRecord {
        id: session_id.clone(),
        device_id: Some(device.id.clone()),
        artifact_id: Some(artifact.id.clone()),
        adapter: "uart_esp_esptool".into(),
        status: "prepared".into(),
        progress: Some(0.0),
        started_at: now,
        finished_at: None,
        error_message: None,
        verified: false,
    };
    db.upsert_flash_session(&record)?;
    log_step(
        db,
        &session_id,
        "prepared",
        "ok",
        "Compatibility and artifact prechecks passed",
    );
    let _ = db.record_operation(
        "flash_prepare",
        Some(&device.id),
        "success",
        &format!("Prepared flash session {}", record.id),
        None,
    );
    let _ = ports;
    Ok(record)
}

pub fn start_flash_session(
    app: &AppHandle,
    db: &Database,
    ports: &PortLockManager,
    device: &DetectedDevice,
    artifact_id: &str,
    session: &FlashSessionRecord,
    confirm: bool,
) -> AppResult<FlashSessionRecord> {
    if !confirm {
        return Err(AppError::Message(
            "Flash requires explicit confirm: true".into(),
        ));
    }
    if device.is_mock {
        return Err(AppError::NotSupported(
            "Mock devices cannot be flashed".into(),
        ));
    }

    let esptool = resolve_esptool().ok_or_else(|| {
        AppError::Message("esptool is not installed or not on PATH".into())
    })?;

    let artifact = db
        .get_firmware_artifact(artifact_id)?
        .ok_or_else(|| AppError::Message("Firmware artifact not found".into()))?;

    let port = device
        .com_port
        .as_deref()
        .ok_or_else(|| AppError::Message("Device has no COM port".into()))?
        .to_string();

    if ports.is_locked(&port) {
        return Err(AppError::PortLocked(port));
    }

    let report = compatibility::evaluate(device, &artifact, Some(0))?;
    if !report.compatible {
        return Err(AppError::Message(
            "Compatibility engine blocked flash start".into(),
        ));
    }

    let validation =
        validate_artifact_file(PathBuf::from(&artifact.file_path).as_path(), Some(&artifact.sha256))?;
    if !validation.valid {
        return Err(AppError::Message(format!(
            "Checksum or file validation failed: {}",
            validation.messages.join("; ")
        )));
    }

    ports.try_lock(&port)?;

    let mut record = session.clone();
    record.status = "running".into();
    record.progress = Some(0.05);
    record.verified = false;
    db.upsert_flash_session(&record)?;

    let steps = |step: &str, progress: f64, message: &str, terminal: bool, status: &str| {
        log_step(db, &record.id, step, status, message);
        emit_progress(
            app,
            FlashProgressEvent {
                session_id: record.id.clone(),
                step: step.into(),
                progress,
                message: message.into(),
                terminal,
            },
        );
    };

    let fail = |record: &mut FlashSessionRecord,
                ports: &PortLockManager,
                port: &str,
                progress: f64,
                err: AppError|
     -> AppResult<FlashSessionRecord> {
        ports.unlock(port);
        record.status = "failed".into();
        record.progress = Some(progress);
        record.finished_at = Some(Utc::now().to_rfc3339());
        record.error_message = Some(err.to_string());
        record.verified = false;
        let _ = db.upsert_flash_session(record);
        log_step(db, &record.id, "failed", "error", &err.to_string());
        emit_progress(
            app,
            FlashProgressEvent {
                session_id: record.id.clone(),
                step: "failed".into(),
                progress,
                message: err.to_string(),
                terminal: true,
            },
        );
        let _ = db.record_operation(
            "flash",
            Some(&device.id),
            "error",
            &format!("Flash session {} failed", record.id),
            Some(&err.to_string()),
        );
        Err(err)
    };

    steps(
        "precheck_device",
        0.08,
        "Device precheck passed",
        false,
        "ok",
    );
    steps(
        "compatibility",
        0.12,
        "Hardware compatibility engine OK",
        false,
        "ok",
    );
    steps(
        "validated_artifact",
        0.2,
        "Artifact checksum OK",
        false,
        "ok",
    );
    steps("port_checked", 0.25, "Serial port locked for flash", false, "ok");
    steps("esptool_ready", 0.3, "esptool located", false, "ok");

    let baud = device.baud_rate.unwrap_or(460_800);

    // USB presence check
    let ports_before = list_com_ports(ports).unwrap_or_default();
    if !ports_before.iter().any(|p| p.name == port) {
        return fail(
            &mut record,
            ports,
            &port,
            0.3,
            AppError::Message("USB/COM port disappeared before flash".into()),
        );
    }

    match esptool_chip_id(&esptool, &port, baud) {
        Ok(out) => {
            steps(
                "connecting",
                0.4,
                &format!("Bootloader probe OK ({})", out.lines().next().unwrap_or("chip_id")),
                false,
                "ok",
            );
        }
        Err(err) => {
            return fail(&mut record, ports, &port, 0.4, err);
        }
    }

    steps("flashing", 0.55, "Writing firmware via esptool…", false, "ok");
    let firmware_path = PathBuf::from(&artifact.file_path);
    let flash_result = esptool_write_flash(&esptool, &port, baud, &firmware_path);

    if let Err(err) = flash_result {
        let msg = format!("esptool write_flash failed: {err}");
        return fail(
            &mut record,
            ports,
            &port,
            0.55,
            AppError::Message(msg),
        );
    }

    steps(
        "post_write_probe",
        0.8,
        "Write process exited 0 — probing device again",
        false,
        "ok",
    );

    // Do NOT trust exit code alone for verified=true
    let mut verified = false;
    let mut completion_note = "Write reported OK by esptool; hardware verification incomplete".to_string();

    // Check USB still present
    let ports_after = list_com_ports(ports).unwrap_or_default();
    if !ports_after.iter().any(|p| p.name == port) {
        steps(
            "port_rescan",
            0.85,
            "COM port missing after write — USB loss suspected",
            false,
            "warn",
        );
        completion_note =
            "Write may have completed but COM port lost; not marking verified".into();
    } else {
        steps("port_rescan", 0.85, "COM port still present", false, "ok");
        match esptool_chip_id(&esptool, &port, baud) {
            Ok(_) => {
                verified = true;
                completion_note =
                    "Post-write bootloader probe succeeded; flash marked verified".into();
                steps(
                    "completed_verified",
                    0.95,
                    &completion_note,
                    false,
                    "ok",
                );
            }
            Err(err) => {
                completion_note = format!(
                    "Post-write probe failed ({err}); write exit was 0 but verified=false"
                );
                steps(
                    "completed_unverified",
                    0.9,
                    &completion_note,
                    false,
                    "warn",
                );
            }
        }
    }

    // Re-check artifact integrity on disk after operation
    if let Ok(v) =
        validate_artifact_file(PathBuf::from(&artifact.file_path).as_path(), Some(&artifact.sha256))
    {
        if !v.valid {
            verified = false;
            completion_note =
                "Local artifact integrity changed during flash — verified cleared".into();
        }
    }

    ports.unlock(&port);

    record.status = if verified {
        "completed_verified".into()
    } else {
        "completed_unverified".into()
    };
    record.progress = Some(1.0);
    record.finished_at = Some(Utc::now().to_rfc3339());
    record.verified = verified;
    record.error_message = if verified {
        None
    } else {
        Some(completion_note.clone())
    };
    db.upsert_flash_session(&record)?;
    steps(
        if verified {
            "completed_verified"
        } else {
            "completed_unverified"
        },
        1.0,
        &completion_note,
        true,
        if verified { "ok" } else { "warn" },
    );
    let _ = db.record_operation(
        "flash",
        Some(&device.id),
        if verified { "success" } else { "partial" },
        &format!("Flash session {} — {}", record.id, completion_note),
        None,
    );
    Ok(record)
}

pub fn pipeline_steps() -> Vec<String> {
    PIPELINE_STEPS.iter().map(|s| s.to_string()).collect()
}
