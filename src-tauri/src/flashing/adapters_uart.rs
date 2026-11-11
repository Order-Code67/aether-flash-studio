use crate::error::{AppError, AppResult};
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
pub struct EsptoolCommand {
    pub executable: String,
}

pub fn resolve_esptool() -> Option<EsptoolCommand> {
    for name in ["esptool", "esptool.py"] {
        let lookup = Command::new(if cfg!(windows) { "where" } else { "which" })
            .arg(name)
            .output()
            .ok();
        if let Some(out) = lookup {
            if out.status.success() {
                let line = String::from_utf8_lossy(&out.stdout)
                    .lines()
                    .next()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(String::from)?;
                return Some(EsptoolCommand { executable: line });
            }
        }
    }
    None
}

pub fn esptool_chip_id(esptool: &EsptoolCommand, port: &str, baud: u32) -> AppResult<String> {
    let output = Command::new(&esptool.executable)
        .args(["--port", port, "--baud", &baud.to_string(), "chip_id"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| AppError::Message(format!("Failed to run esptool: {e}")))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if output.status.success() {
        Ok(stdout.to_string())
    } else {
        Err(AppError::Message(format!(
            "esptool chip_id failed: {}",
            if !stderr.trim().is_empty() {
                stderr.trim()
            } else {
                stdout.trim()
            }
        )))
    }
}

pub fn esptool_write_flash(
    esptool: &EsptoolCommand,
    port: &str,
    baud: u32,
    firmware: &Path,
) -> AppResult<()> {
    let output = Command::new(&esptool.executable)
        .args([
            "--port",
            port,
            "--baud",
            &baud.to_string(),
            "write_flash",
            "0x0",
            firmware.to_string_lossy().as_ref(),
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| AppError::Message(format!("Failed to run esptool write_flash: {e}")))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        Err(AppError::Message(format!(
            "esptool write_flash failed: {}",
            if !stderr.trim().is_empty() {
                stderr.trim()
            } else {
                stdout.trim()
            }
        )))
    }
}
