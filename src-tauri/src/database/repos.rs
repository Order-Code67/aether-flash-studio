use super::Database;
use crate::error::{AppError, AppResult};
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareArtifactRecord {
    pub id: String,
    pub firmware_version_id: String,
    pub hardware_target_id: Option<String>,
    pub file_path: String,
    pub sha256: String,
    pub file_size: i64,
    pub verified: bool,
    pub manufacturer: String,
    pub radio_system: String,
    pub version: String,
    pub channel: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareBuildRecord {
    pub id: String,
    pub source_repo: Option<String>,
    pub branch: Option<String>,
    pub target: Option<String>,
    pub config_json: Option<String>,
    pub status: String,
    pub artifact_id: Option<String>,
    pub log_path: Option<String>,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupRecord {
    pub id: String,
    pub device_id: Option<String>,
    pub backup_type: String,
    pub file_path: Option<String>,
    pub sha256: Option<String>,
    pub model: Option<String>,
    pub firmware_version: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlashSessionRecord {
    pub id: String,
    pub device_id: Option<String>,
    pub artifact_id: Option<String>,
    pub adapter: String,
    pub status: String,
    pub progress: Option<f64>,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub error_message: Option<String>,
    pub verified: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportFirmwareMeta {
    pub manufacturer: String,
    pub radio_system: String,
    pub version: String,
    pub channel: Option<String>,
    pub source: Option<String>,
    pub release_notes: Option<String>,
    pub hardware_target_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareArtifactFilter {
    pub manufacturer: Option<String>,
    pub radio_system: Option<String>,
    pub version: Option<String>,
    pub verified_only: Option<bool>,
    pub limit: Option<i64>,
}

impl Database {
    pub fn insert_firmware_version(
        &self,
        manufacturer: &str,
        radio_system: &str,
        version: &str,
        channel: &str,
        source: Option<&str>,
        release_notes: Option<&str>,
    ) -> AppResult<String> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        conn.execute(
            "INSERT INTO firmware_versions (id, manufacturer, radio_system, version, channel, source, release_notes, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
             ON CONFLICT(manufacturer, radio_system, version, channel) DO UPDATE SET
                source = COALESCE(excluded.source, firmware_versions.source),
                release_notes = COALESCE(excluded.release_notes, firmware_versions.release_notes)",
            params![
                id,
                manufacturer,
                radio_system,
                version,
                channel,
                source,
                release_notes,
                now
            ],
        )?;
        let resolved: String = conn.query_row(
            "SELECT id FROM firmware_versions WHERE manufacturer = ?1 AND radio_system = ?2 AND version = ?3 AND channel = ?4",
            params![manufacturer, radio_system, version, channel],
            |row| row.get(0),
        )?;
        Ok(resolved)
    }

    pub fn insert_firmware_artifact(
        &self,
        firmware_version_id: &str,
        hardware_target_id: Option<&str>,
        file_path: &str,
        sha256: &str,
        file_size: i64,
        verified: bool,
    ) -> AppResult<String> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        conn.execute(
            "INSERT INTO firmware_artifacts (id, firmware_version_id, hardware_target_id, file_path, sha256, file_size, verified, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                id,
                firmware_version_id,
                hardware_target_id,
                file_path,
                sha256,
                file_size,
                verified as i64,
                now
            ],
        )?;
        Ok(id)
    }

    pub fn get_firmware_artifact(&self, id: &str) -> AppResult<Option<FirmwareArtifactRecord>> {
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT fa.id, fa.firmware_version_id, fa.hardware_target_id, fa.file_path, fa.sha256, fa.file_size, fa.verified, fa.created_at,
                    fv.manufacturer, fv.radio_system, fv.version, fv.channel
             FROM firmware_artifacts fa
             JOIN firmware_versions fv ON fv.id = fa.firmware_version_id
             WHERE fa.id = ?1",
        )?;
        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(FirmwareArtifactRecord {
                id: row.get(0)?,
                firmware_version_id: row.get(1)?,
                hardware_target_id: row.get(2)?,
                file_path: row.get(3)?,
                sha256: row.get(4)?,
                file_size: row.get(5)?,
                verified: row.get::<_, i64>(6)? != 0,
                created_at: row.get(7)?,
                manufacturer: row.get(8)?,
                radio_system: row.get(9)?,
                version: row.get(10)?,
                channel: row.get(11)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn list_firmware_artifacts(
        &self,
        filter: &FirmwareArtifactFilter,
    ) -> AppResult<Vec<FirmwareArtifactRecord>> {
        let limit = filter.limit.unwrap_or(100);
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        let mut sql = String::from(
            "SELECT fa.id, fa.firmware_version_id, fa.hardware_target_id, fa.file_path, fa.sha256, fa.file_size, fa.verified, fa.created_at,
                    fv.manufacturer, fv.radio_system, fv.version, fv.channel
             FROM firmware_artifacts fa
             JOIN firmware_versions fv ON fv.id = fa.firmware_version_id
             WHERE 1=1",
        );
        let mut binds: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        if let Some(m) = &filter.manufacturer {
            sql.push_str(" AND fv.manufacturer LIKE ?");
            binds.push(Box::new(format!("%{}%", m)));
        }
        if let Some(r) = &filter.radio_system {
            sql.push_str(" AND fv.radio_system LIKE ?");
            binds.push(Box::new(format!("%{}%", r)));
        }
        if let Some(v) = &filter.version {
            sql.push_str(" AND fv.version LIKE ?");
            binds.push(Box::new(format!("%{}%", v)));
        }
        if filter.verified_only.unwrap_or(false) {
            sql.push_str(" AND fa.verified = 1");
        }
        sql.push_str(" ORDER BY fa.created_at DESC LIMIT ?");
        binds.push(Box::new(limit));

        let refs: Vec<&dyn rusqlite::ToSql> = binds.iter().map(|b| b.as_ref()).collect();
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(refs.as_slice(), |row| {
            Ok(FirmwareArtifactRecord {
                id: row.get(0)?,
                firmware_version_id: row.get(1)?,
                hardware_target_id: row.get(2)?,
                file_path: row.get(3)?,
                sha256: row.get(4)?,
                file_size: row.get(5)?,
                verified: row.get::<_, i64>(6)? != 0,
                created_at: row.get(7)?,
                manufacturer: row.get(8)?,
                radio_system: row.get(9)?,
                version: row.get(10)?,
                channel: row.get(11)?,
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn set_firmware_artifact_verified(&self, id: &str, verified: bool) -> AppResult<()> {
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        conn.execute(
            "UPDATE firmware_artifacts SET verified = ?2 WHERE id = ?1",
            params![id, verified as i64],
        )?;
        Ok(())
    }

    pub fn insert_firmware_build(
        &self,
        source_repo: Option<&str>,
        branch: Option<&str>,
        target: Option<&str>,
        config_json: Option<&str>,
        status: &str,
        error_message: Option<&str>,
    ) -> AppResult<String> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        conn.execute(
            "INSERT INTO firmware_builds (id, source_repo, branch, target, config_json, status, started_at, error_message)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![id, source_repo, branch, target, config_json, status, now, error_message],
        )?;
        Ok(id)
    }

    pub fn update_firmware_build(
        &self,
        id: &str,
        status: &str,
        artifact_id: Option<&str>,
        log_path: Option<&str>,
        error_message: Option<&str>,
    ) -> AppResult<()> {
        let now = Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        conn.execute(
            "UPDATE firmware_builds SET status = ?2, artifact_id = ?3, log_path = ?4, finished_at = ?5, error_message = ?6 WHERE id = ?1",
            params![id, status, artifact_id, log_path, now, error_message],
        )?;
        Ok(())
    }

    pub fn get_firmware_build(&self, id: &str) -> AppResult<Option<FirmwareBuildRecord>> {
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, source_repo, branch, target, config_json, status, artifact_id, log_path, started_at, finished_at, error_message
             FROM firmware_builds WHERE id = ?1",
        )?;
        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(FirmwareBuildRecord {
                id: row.get(0)?,
                source_repo: row.get(1)?,
                branch: row.get(2)?,
                target: row.get(3)?,
                config_json: row.get(4)?,
                status: row.get(5)?,
                artifact_id: row.get(6)?,
                log_path: row.get(7)?,
                started_at: row.get(8)?,
                finished_at: row.get(9)?,
                error_message: row.get(10)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn insert_backup(
        &self,
        device_id: Option<&str>,
        backup_type: &str,
        file_path: Option<&str>,
        sha256: Option<&str>,
        model: Option<&str>,
        firmware_version: Option<&str>,
        description: Option<&str>,
    ) -> AppResult<String> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        conn.execute(
            "INSERT INTO backups (id, device_id, backup_type, file_path, sha256, model, firmware_version, description, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                id,
                device_id,
                backup_type,
                file_path,
                sha256,
                model,
                firmware_version,
                description,
                now
            ],
        )?;
        Ok(id)
    }

    pub fn list_backups(&self, limit: i64) -> AppResult<Vec<BackupRecord>> {
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, device_id, backup_type, file_path, sha256, model, firmware_version, description, created_at
             FROM backups ORDER BY created_at DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit], |row| {
            Ok(BackupRecord {
                id: row.get(0)?,
                device_id: row.get(1)?,
                backup_type: row.get(2)?,
                file_path: row.get(3)?,
                sha256: row.get(4)?,
                model: row.get(5)?,
                firmware_version: row.get(6)?,
                description: row.get(7)?,
                created_at: row.get(8)?,
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn get_backup(&self, id: &str) -> AppResult<Option<BackupRecord>> {
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, device_id, backup_type, file_path, sha256, model, firmware_version, description, created_at
             FROM backups WHERE id = ?1",
        )?;
        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(BackupRecord {
                id: row.get(0)?,
                device_id: row.get(1)?,
                backup_type: row.get(2)?,
                file_path: row.get(3)?,
                sha256: row.get(4)?,
                model: row.get(5)?,
                firmware_version: row.get(6)?,
                description: row.get(7)?,
                created_at: row.get(8)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn delete_backup(&self, id: &str) -> AppResult<()> {
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        conn.execute("DELETE FROM backups WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn insert_device_configuration(
        &self,
        id: &str,
        device_id: Option<&str>,
        profile_id: Option<&str>,
        category: &str,
        payload_json: &str,
        firmware_version: Option<&str>,
        created_at: &str,
    ) -> AppResult<()> {
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        conn.execute(
            "INSERT INTO device_configurations
             (id, device_id, profile_id, category, payload_json, firmware_version, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                id,
                device_id,
                profile_id,
                category,
                payload_json,
                firmware_version,
                created_at
            ],
        )?;
        Ok(())
    }

    pub fn upsert_source_download(
        &self,
        project: &str,
        version_or_ref: &str,
        source_kind: &str,
        remote_url: &str,
        local_path: &str,
        commit_sha: Option<&str>,
        integrity_ok: bool,
        notes: Option<&str>,
    ) -> AppResult<crate::elrs_sources::SourceDownloadRecord> {
        let now = Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        let existing: Option<(String, String)> = conn
            .query_row(
                "SELECT id, downloaded_at FROM source_downloads WHERE project = ?1 AND version_or_ref = ?2 AND local_path = ?3",
                params![project, version_or_ref, local_path],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .ok();
        let (id, downloaded_at) = existing.unwrap_or_else(|| (uuid::Uuid::new_v4().to_string(), now.clone()));
        conn.execute(
            "INSERT INTO source_downloads
             (id, project, version_or_ref, source_kind, remote_url, local_path, commit_sha, integrity_ok, notes, downloaded_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
             ON CONFLICT(project, version_or_ref, local_path) DO UPDATE SET
                source_kind = excluded.source_kind,
                remote_url = excluded.remote_url,
                commit_sha = excluded.commit_sha,
                integrity_ok = excluded.integrity_ok,
                notes = excluded.notes,
                updated_at = excluded.updated_at",
            params![
                id,
                project,
                version_or_ref,
                source_kind,
                remote_url,
                local_path,
                commit_sha,
                integrity_ok as i64,
                notes,
                downloaded_at,
                now
            ],
        )?;
        Ok(crate::elrs_sources::SourceDownloadRecord {
            id,
            project: project.into(),
            version_or_ref: version_or_ref.into(),
            source_kind: source_kind.into(),
            remote_url: remote_url.into(),
            local_path: local_path.into(),
            commit_sha: commit_sha.map(|s| s.to_string()),
            integrity_ok,
            notes: notes.map(|s| s.to_string()),
            downloaded_at,
            updated_at: now,
        })
    }

    pub fn list_source_downloads(
        &self,
        project: &str,
    ) -> AppResult<Vec<crate::elrs_sources::SourceDownloadRecord>> {
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, project, version_or_ref, source_kind, remote_url, local_path, commit_sha, integrity_ok, notes, downloaded_at, updated_at
             FROM source_downloads WHERE project = ?1 ORDER BY updated_at DESC",
        )?;
        let rows = stmt.query_map(params![project], |row| {
            Ok(crate::elrs_sources::SourceDownloadRecord {
                id: row.get(0)?,
                project: row.get(1)?,
                version_or_ref: row.get(2)?,
                source_kind: row.get(3)?,
                remote_url: row.get(4)?,
                local_path: row.get(5)?,
                commit_sha: row.get(6)?,
                integrity_ok: row.get::<_, i64>(7)? != 0,
                notes: row.get(8)?,
                downloaded_at: row.get(9)?,
                updated_at: row.get(10)?,
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn insert_flash_step_log(
        &self,
        session_id: &str,
        step: &str,
        status: &str,
        message: Option<&str>,
    ) -> AppResult<()> {
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        conn.execute(
            "INSERT INTO flash_step_logs (id, session_id, step, status, message, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                uuid::Uuid::new_v4().to_string(),
                session_id,
                step,
                status,
                message,
                Utc::now().to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn list_flash_step_logs(
        &self,
        session_id: &str,
    ) -> AppResult<Vec<serde_json::Value>> {
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT step, status, message, created_at FROM flash_step_logs
             WHERE session_id = ?1 ORDER BY created_at ASC",
        )?;
        let rows = stmt.query_map(params![session_id], |row| {
            Ok(serde_json::json!({
                "step": row.get::<_, String>(0)?,
                "status": row.get::<_, String>(1)?,
                "message": row.get::<_, Option<String>>(2)?,
                "createdAt": row.get::<_, String>(3)?,
            }))
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn list_device_configurations(
        &self,
        device_id: Option<&str>,
    ) -> AppResult<Vec<crate::configuration::StoredConfiguration>> {
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        let mut sql = String::from(
            "SELECT id, device_id, profile_id, category, payload_json, firmware_version, created_at
             FROM device_configurations",
        );
        if device_id.is_some() {
            sql.push_str(" WHERE device_id = ?1");
        }
        sql.push_str(" ORDER BY created_at DESC LIMIT 200");
        let mut stmt = conn.prepare(&sql)?;
        let map_row = |row: &rusqlite::Row| {
            let payload_raw: String = row.get(4)?;
            let payload = serde_json::from_str(&payload_raw).unwrap_or(serde_json::json!({}));
            Ok(crate::configuration::StoredConfiguration {
                id: row.get(0)?,
                device_id: row.get(1)?,
                profile_id: row.get(2)?,
                category: row.get(3)?,
                payload,
                firmware_version: row.get(5)?,
                created_at: row.get(6)?,
            })
        };
        let rows = if let Some(id) = device_id {
            stmt.query_map(params![id], map_row)?
        } else {
            stmt.query_map([], map_row)?
        };
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn upsert_flash_session(&self, record: &FlashSessionRecord) -> AppResult<()> {
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        conn.execute(
            "INSERT INTO flash_sessions (id, device_id, artifact_id, adapter, status, progress, started_at, finished_at, error_message, verified)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
             ON CONFLICT(id) DO UPDATE SET
                status = excluded.status,
                progress = excluded.progress,
                finished_at = excluded.finished_at,
                error_message = excluded.error_message,
                verified = excluded.verified",
            params![
                record.id,
                record.device_id,
                record.artifact_id,
                record.adapter,
                record.status,
                record.progress,
                record.started_at,
                record.finished_at,
                record.error_message,
                record.verified as i64
            ],
        )?;
        Ok(())
    }

    pub fn get_flash_session(&self, id: &str) -> AppResult<Option<FlashSessionRecord>> {
        let conn = self.conn.lock().map_err(|e| AppError::Message(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, device_id, artifact_id, adapter, status, progress, started_at, finished_at, error_message, verified
             FROM flash_sessions WHERE id = ?1",
        )?;
        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(FlashSessionRecord {
                id: row.get(0)?,
                device_id: row.get(1)?,
                artifact_id: row.get(2)?,
                adapter: row.get(3)?,
                status: row.get(4)?,
                progress: row.get(5)?,
                started_at: row.get(6)?,
                finished_at: row.get(7)?,
                error_message: row.get(8)?,
                verified: row.get::<_, i64>(9)? != 0,
            }))
        } else {
            Ok(None)
        }
    }
}
