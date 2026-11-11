use crate::database::Database;
use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildToolStatus {
    pub platformio: ToolPresence,
    pub esptool: ToolPresence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolPresence {
    pub available: bool,
    pub command: Option<String>,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareBuildRequest {
    pub source_path: Option<String>,
    pub source_repo: Option<String>,
    pub branch: Option<String>,
    pub target: Option<String>,
    pub config: Option<serde_json::Value>,
}

fn which_first(candidates: &[&str]) -> Option<String> {
    for name in candidates {
        let output = Command::new(if cfg!(windows) { "where" } else { "which" }).arg(name).output();
        if let Ok(out) = output {
            if out.status.success() {
                let line = String::from_utf8_lossy(&out.stdout)
                    .lines()
                    .next()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(String::from);
                if line.is_some() {
                    return line;
                }
            }
        }
    }
    None
}

pub fn check_build_tools() -> BuildToolStatus {
    let pio = which_first(&["pio", "platformio"]);
    let esptool = which_first(&["esptool", "esptool.py"]);

    BuildToolStatus {
        platformio: ToolPresence {
            available: pio.is_some(),
            command: pio.clone(),
            detail: if pio.is_some() {
                Some("PlatformIO CLI detected on PATH".into())
            } else {
                Some("Install PlatformIO (pio) and ensure it is on PATH".into())
            },
        },
        esptool: ToolPresence {
            available: esptool.is_some(),
            command: esptool.clone(),
            detail: if esptool.is_some() {
                Some("esptool detected on PATH".into())
            } else {
                Some("Install esptool.py and ensure it is on PATH".into())
            },
        },
    }
}

pub fn dry_validate_build_config(config: &serde_json::Value) -> AppResult<()> {
    if !config.is_object() {
        return Err(AppError::Message(
            "Build config must be a JSON object for dry validation".into(),
        ));
    }
    Ok(())
}

pub fn start_firmware_build(db: &Database, request: FirmwareBuildRequest) -> AppResult<String> {
    let tools = check_build_tools();
    let config_json = request
        .config
        .as_ref()
        .map(|c| serde_json::to_string(c))
        .transpose()
        .map_err(|e| AppError::Message(e.to_string()))?;

    if let Some(ref cfg) = request.config {
        dry_validate_build_config(cfg)?;
    }

    let build_id = db.insert_firmware_build(
        request.source_repo.as_deref(),
        request.branch.as_deref(),
        request.target.as_deref(),
        config_json.as_deref(),
        "pending",
        None,
    )?;

    let source_path = request
        .source_path
        .as_deref()
        .or(request.source_repo.as_deref());

    let Some(path_str) = source_path else {
        db.update_firmware_build(
            &build_id,
            "failed",
            None,
            None,
            Some("Provide source_path or source_repo for a build"),
        )?;
        return Err(AppError::Message(
            "Provide source_path (local PlatformIO project) or source_repo".into(),
        ));
    };

    let path = Path::new(path_str);
    if !path.exists() {
        let msg = format!("Source path does not exist: {}", path.display());
        db.update_firmware_build(&build_id, "failed", None, None, Some(&msg))?;
        return Err(AppError::Message(msg));
    }

    if !tools.platformio.available {
        let msg = "PlatformIO (pio) is not installed or not on PATH";
        db.update_firmware_build(&build_id, "failed", None, None, Some(msg))?;
        return Err(AppError::Message(msg.into()));
    }

    let pio_cmd = tools.platformio.command.as_deref().unwrap_or("pio");
    let mut cmd = Command::new(pio_cmd);
    cmd.arg("run").current_dir(path);
    if let Some(target) = &request.target {
        cmd.arg("-e").arg(target);
    }

    db.update_firmware_build(&build_id, "running", None, None, None)?;

    let output = cmd.output().map_err(|e| {
        let msg = format!("Failed to start PlatformIO build: {e}");
        let _ = db.update_firmware_build(&build_id, "failed", None, None, Some(&msg));
        AppError::Message(msg)
    })?;

    if output.status.success() {
        db.update_firmware_build(&build_id, "completed", None, None, None)?;
        let _ = db.record_operation(
            "firmware_build",
            None,
            "success",
            &format!("Build {build_id} completed"),
            None,
        );
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let msg = if !stderr.trim().is_empty() {
            stderr.trim().to_string()
        } else {
            stdout.trim().to_string()
        };
        let summary = if msg.len() > 500 {
            format!("{}…", &msg[..500])
        } else {
            msg.clone()
        };
        db.update_firmware_build(&build_id, "failed", None, None, Some(&summary))?;
        let _ = db.record_operation(
            "firmware_build",
            None,
            "error",
            &format!("Build {build_id} failed"),
            Some(&summary),
        );
        return Err(AppError::Message(format!("PlatformIO build failed: {summary}")));
    }

    Ok(build_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dry_validate_rejects_non_object() {
        assert!(dry_validate_build_config(&serde_json::json!([])).is_err());
        assert!(dry_validate_build_config(&serde_json::json!({ "env": "tx" })).is_ok());
    }
}
