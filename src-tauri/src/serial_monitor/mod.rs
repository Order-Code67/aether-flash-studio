use crate::error::{AppError, AppResult};
use crate::serial::PortLockManager;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

#[derive(Default)]
pub struct SerialSessionHub {
    active: Mutex<Option<ActiveSession>>,
}

struct ActiveSession {
    port_name: String,
    // Keep port open behind mutex for send/read loop ownership
    port: Arc<Mutex<Box<dyn serialport::SerialPort>>>,
    stop: Arc<std::sync::atomic::AtomicBool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenSerialRequest {
    pub com_port: String,
    pub baud_rate: u32,
    pub mode: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SerialDataEvent {
    pub com_port: String,
    pub mode: String,
    pub chunk: String,
    pub hex: bool,
}

impl SerialSessionHub {
    pub fn open(
        &self,
        app: AppHandle,
        locks: &PortLockManager,
        req: OpenSerialRequest,
    ) -> AppResult<()> {
        if self.active.lock().is_some() {
            return Err(AppError::Message(
                "A serial monitor session is already open".into(),
            ));
        }
        locks.try_lock(&req.com_port)?;
        let port = serialport::new(&req.com_port, req.baud_rate)
            .timeout(Duration::from_millis(200))
            .open()
            .map_err(|e| {
                locks.unlock(&req.com_port);
                AppError::Serial(e.to_string())
            })?;
        let port = Arc::new(Mutex::new(port));
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let reader_port = Arc::clone(&port);
        let reader_stop = Arc::clone(&stop);
        let com = req.com_port.clone();
        let mode = req.mode.clone();
        let hex_mode = mode.eq_ignore_ascii_case("hex");

        std::thread::spawn(move || {
            let mut buf = [0u8; 512];
            while !reader_stop.load(std::sync::atomic::Ordering::Relaxed) {
                let read = {
                    let mut guard = reader_port.lock();
                    guard.read(&mut buf)
                };
                match read {
                    Ok(0) => {}
                    Ok(n) => {
                        let chunk = if hex_mode {
                            buf[..n]
                                .iter()
                                .map(|b| format!("{b:02X}"))
                                .collect::<Vec<_>>()
                                .join(" ")
                        } else {
                            String::from_utf8_lossy(&buf[..n]).to_string()
                        };
                        let _ = app.emit(
                            "serial-data",
                            SerialDataEvent {
                                com_port: com.clone(),
                                mode: mode.clone(),
                                chunk,
                                hex: hex_mode,
                            },
                        );
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {}
                    Err(_) => break,
                }
            }
        });

        *self.active.lock() = Some(ActiveSession {
            port_name: req.com_port,
            port,
            stop,
        });
        Ok(())
    }

    pub fn close(&self, locks: &PortLockManager) -> AppResult<()> {
        let mut guard = self.active.lock();
        if let Some(session) = guard.take() {
            session
                .stop
                .store(true, std::sync::atomic::Ordering::Relaxed);
            locks.unlock(&session.port_name);
        }
        Ok(())
    }

    pub fn send(&self, data: &str, hex: bool) -> AppResult<()> {
        let guard = self.active.lock();
        let session = guard
            .as_ref()
            .ok_or_else(|| AppError::Message("Serial monitor is not open".into()))?;
        let bytes = if hex {
            parse_hex_bytes(data)?
        } else {
            data.as_bytes().to_vec()
        };
        session
            .port
            .lock()
            .write_all(&bytes)
            .map_err(|e| AppError::Serial(e.to_string()))?;
        Ok(())
    }

    pub fn active_port(&self) -> Option<String> {
        self.active.lock().as_ref().map(|s| s.port_name.clone())
    }
}

fn parse_hex_bytes(data: &str) -> AppResult<Vec<u8>> {
    let cleaned: String = data.chars().filter(|c| !c.is_whitespace()).collect();
    if cleaned.len() % 2 != 0 {
        return Err(AppError::Message("HEX payload length must be even".into()));
    }
    (0..cleaned.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&cleaned[i..i + 2], 16)
                .map_err(|_| AppError::Message("Invalid HEX payload".into()))
        })
        .collect()
}

pub mod crsf {
    use serde::Serialize;

    #[derive(Debug, Clone, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CrsfFrameView {
        pub frame_type: String,
        pub summary: String,
    }

    /// Decode only a few well-known CRSF frame type bytes from a raw buffer.
    pub fn decode_preview(bytes: &[u8]) -> Vec<CrsfFrameView> {
        let mut out = Vec::new();
        let mut i = 0;
        while i + 2 < bytes.len() {
            if bytes[i] != 0xC8 {
                i += 1;
                continue;
            }
            let len = bytes[i + 1] as usize;
            if i + 2 + len > bytes.len() {
                break;
            }
            let type_byte = bytes[i + 2];
            let frame_type = match type_byte {
                0x16 => "RC_CHANNELS_PACKED",
                0x14 => "LINK_STATISTICS",
                0x08 => "BATTERY_SENSOR",
                0x21 => "DEVICE_INFO",
                _ => "UNKNOWN",
            };
            if frame_type != "UNKNOWN" {
                out.push(CrsfFrameView {
                    frame_type: frame_type.into(),
                    summary: format!("type=0x{type_byte:02X} len={len}"),
                });
            }
            i += 2 + len;
        }
        out
    }
}
