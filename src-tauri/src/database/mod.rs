mod migrations;
mod repos;

pub use repos::*;

use crate::error::{AppError, AppResult};
use chrono::Utc;
use rusqlite::{params, Connection};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

pub struct Database {
    conn: Mutex<Connection>,
    path: PathBuf,
}

impl Database {
    pub fn open(path: impl AsRef<Path>) -> AppResult<Self> {
        if let Some(parent) = path.as_ref().parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(path.as_ref())?;
        conn.execute_batch(
            "PRAGMA foreign_keys = ON;
             PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;",
        )?;

        let db = Self {
            conn: Mutex::new(conn),
            path: path.as_ref().to_path_buf(),
        };
        db.migrate()?;
        db.seed_defaults()?;
        Ok(db)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    fn migrate(&self) -> AppResult<()> {
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS schema_migrations (
                version INTEGER PRIMARY KEY NOT NULL,
                applied_at TEXT NOT NULL
            );",
        )?;

        let current: i64 = conn
            .query_row(
                "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        for (idx, sql) in migrations::MIGRATIONS.iter().enumerate() {
            let version = (idx + 1) as i64;
            if version <= current {
                continue;
            }
            conn.execute_batch(sql)?;
            conn.execute(
                "INSERT INTO schema_migrations (version, applied_at) VALUES (?1, ?2)",
                params![version, Utc::now().to_rfc3339()],
            )?;
            tracing::info!(category = "INFO", version, "Applied database migration");
        }
        Ok(())
    }

    fn seed_defaults(&self) -> AppResult<()> {
        let now = Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;

        conn.execute(
            "INSERT OR IGNORE INTO manufacturers (id, name, website, notes, created_at)
             VALUES ('mfr-expresslrs', 'ExpressLRS', 'https://www.expresslrs.org', 'Primary Phase 1 target', ?1)",
            params![now],
        )?;

        let defaults = [
            ("mock_device_mode", "false"),
            ("app_version", "0.3.0"),
            ("last_firmware_used", ""),
            ("uart_default_baud", "460800"),
            ("ui_locale", "ru"),
        ];

        for (key, value) in defaults {
            conn.execute(
                "INSERT OR IGNORE INTO application_settings (key, value, updated_at) VALUES (?1, ?2, ?3)",
                params![key, value, now],
            )?;
        }
        Ok(())
    }

    pub fn get_setting(&self, key: &str) -> AppResult<Option<String>> {
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        let mut stmt = conn.prepare("SELECT value FROM application_settings WHERE key = ?1")?;
        let mut rows = stmt.query(params![key])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn set_setting(&self, key: &str, value: &str) -> AppResult<()> {
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        conn.execute(
            "INSERT INTO application_settings (key, value, updated_at)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
            params![key, value, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn record_operation(
        &self,
        operation_type: &str,
        device_id: Option<&str>,
        status: &str,
        description: &str,
        error_message: Option<&str>,
    ) -> AppResult<String> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        conn.execute(
            "INSERT INTO operation_history
             (id, operation_type, device_id, status, started_at, finished_at, description, error_message)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                id,
                operation_type,
                device_id,
                status,
                now,
                now,
                description,
                error_message
            ],
        )?;
        Ok(id)
    }

    pub fn recent_operations(&self, limit: i64) -> AppResult<Vec<OperationRecord>> {
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, operation_type, device_id, status, started_at, finished_at, description, error_message
             FROM operation_history
             ORDER BY started_at DESC
             LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit], |row| {
            Ok(OperationRecord {
                id: row.get(0)?,
                operation_type: row.get(1)?,
                device_id: row.get(2)?,
                status: row.get(3)?,
                started_at: row.get(4)?,
                finished_at: row.get(5)?,
                description: row.get(6)?,
                error_message: row.get(7)?,
            })
        })?;

        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn list_device_profiles(&self) -> AppResult<Vec<crate::device_detection::DeviceProfile>> {
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, device_id, name, manufacturer, model, notes, preferred_com_port, preferred_baud, created_at, updated_at
             FROM device_profiles ORDER BY updated_at DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(crate::device_detection::DeviceProfile {
                id: row.get(0)?,
                device_id: row.get(1)?,
                name: row.get(2)?,
                manufacturer: row.get(3)?,
                model: row.get(4)?,
                notes: row.get(5)?,
                preferred_com_port: row.get(6)?,
                preferred_baud: row
                    .get::<_, Option<i64>>(7)?
                    .map(|v| v as u32),
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn upsert_device_profile(
        &self,
        input: crate::device_detection::UpsertDeviceProfile,
    ) -> AppResult<crate::device_detection::DeviceProfile> {
        let now = Utc::now().to_rfc3339();
        let id = input
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;

        let existing: Option<String> = conn
            .query_row(
                "SELECT created_at FROM device_profiles WHERE id = ?1",
                params![id],
                |row| row.get(0),
            )
            .ok();
        let created_at = existing.unwrap_or_else(|| now.clone());

        conn.execute(
            "INSERT INTO device_profiles
             (id, device_id, name, manufacturer, model, notes, preferred_com_port, preferred_baud, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
             ON CONFLICT(id) DO UPDATE SET
                device_id = excluded.device_id,
                name = excluded.name,
                manufacturer = excluded.manufacturer,
                model = excluded.model,
                notes = excluded.notes,
                preferred_com_port = excluded.preferred_com_port,
                preferred_baud = excluded.preferred_baud,
                updated_at = excluded.updated_at",
            params![
                id,
                input.device_id,
                input.name,
                input.manufacturer,
                input.model,
                input.notes,
                input.preferred_com_port,
                input.preferred_baud.map(|v| v as i64),
                created_at,
                now
            ],
        )?;

        Ok(crate::device_detection::DeviceProfile {
            id,
            device_id: input.device_id,
            name: input.name,
            manufacturer: input.manufacturer,
            model: input.model,
            notes: input.notes,
            preferred_com_port: input.preferred_com_port,
            preferred_baud: input.preferred_baud,
            created_at,
            updated_at: now,
        })
    }

    pub fn delete_device_profile(&self, id: &str) -> AppResult<()> {
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        conn.execute("DELETE FROM device_profiles WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn find_profile_for_device(
        &self,
        device_id: &str,
    ) -> AppResult<Option<crate::device_detection::DeviceProfile>> {
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, device_id, name, manufacturer, model, notes, preferred_com_port, preferred_baud, created_at, updated_at
             FROM device_profiles WHERE device_id = ?1 ORDER BY updated_at DESC LIMIT 1",
        )?;
        let mut rows = stmt.query(params![device_id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(crate::device_detection::DeviceProfile {
                id: row.get(0)?,
                device_id: row.get(1)?,
                name: row.get(2)?,
                manufacturer: row.get(3)?,
                model: row.get(4)?,
                notes: row.get(5)?,
                preferred_com_port: row.get(6)?,
                preferred_baud: row
                    .get::<_, Option<i64>>(7)?
                    .map(|v| v as u32),
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn upsert_device_snapshot(&self, device: &crate::device_detection::DetectedDevice) -> AppResult<()> {
        let now = Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        conn.execute(
            "INSERT INTO devices (
                id, name, manufacturer, device_type, model, mcu, com_port, vid, pid, usb_serial,
                radio_system, frequency_band, firmware_version, bootloader_version, connection_mode,
                flash_method, status, is_mock, last_seen_at, created_at, updated_at
             ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
                ?11, ?12, ?13, ?14, ?15,
                ?16, ?17, ?18, ?19, ?20, ?21
             )
             ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                manufacturer = excluded.manufacturer,
                device_type = excluded.device_type,
                model = excluded.model,
                mcu = excluded.mcu,
                com_port = excluded.com_port,
                vid = excluded.vid,
                pid = excluded.pid,
                usb_serial = excluded.usb_serial,
                radio_system = excluded.radio_system,
                frequency_band = excluded.frequency_band,
                firmware_version = excluded.firmware_version,
                bootloader_version = excluded.bootloader_version,
                connection_mode = excluded.connection_mode,
                flash_method = excluded.flash_method,
                status = excluded.status,
                is_mock = excluded.is_mock,
                last_seen_at = excluded.last_seen_at,
                updated_at = excluded.updated_at",
            params![
                device.id,
                device.name,
                device.manufacturer,
                device.device_type.as_str(),
                device.model,
                device.mcu,
                device.com_port,
                device.vid.map(|v| v as i64),
                device.pid.map(|v| v as i64),
                device.usb_serial,
                device.radio_system,
                device.frequency_band,
                device.firmware_version,
                device.bootloader_version,
                device.connection_mode,
                device.flash_method,
                device.status.as_str(),
                device.is_mock as i64,
                now,
                now,
                now
            ],
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationRecord {
    pub id: String,
    pub operation_type: String,
    pub device_id: Option<String>,
    pub status: String,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub description: String,
    pub error_message: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn migrations_and_settings_work() {
        let dir = tempdir().unwrap();
        let db = Database::open(dir.path().join("test.db")).unwrap();
        db.set_setting("mock_device_mode", "true").unwrap();
        assert_eq!(
            db.get_setting("mock_device_mode").unwrap().as_deref(),
            Some("true")
        );
        let id = db
            .record_operation("scan", None, "success", "scan completed", None)
            .unwrap();
        assert!(!id.is_empty());
        assert_eq!(db.recent_operations(10).unwrap().len(), 1);
    }
}
