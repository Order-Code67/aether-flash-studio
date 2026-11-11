use crate::database::FirmwareArtifactRecord;
use crate::error::{AppError, AppResult};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

const ALLOWED_EXTENSIONS: &[&str] = &["bin", "elf", "hex"];

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationResult {
    pub valid: bool,
    pub sha256: String,
    pub file_size: u64,
    pub extension: String,
    pub messages: Vec<String>,
}

pub fn extension_allowed(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| {
            let lower = e.to_lowercase();
            ALLOWED_EXTENSIONS.contains(&lower.as_str())
        })
        .unwrap_or(false)
}

pub fn compute_sha256(path: &Path) -> AppResult<(String, u64)> {
    let file = File::open(path)?;
    let len = file.metadata()?.len();
    if len == 0 {
        return Err(AppError::Message("Firmware file is empty".into()));
    }
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    let digest = hex::encode(hasher.finalize());
    Ok((digest, len))
}

pub fn validate_artifact_file(path: &Path, expected_sha256: Option<&str>) -> AppResult<ValidationResult> {
    if !path.exists() {
        return Err(AppError::Message(format!(
            "Firmware artifact not found: {}",
            path.display()
        )));
    }
    let mut messages = Vec::new();
    if !extension_allowed(path) {
        messages.push(format!(
            "Extension must be one of: {}",
            ALLOWED_EXTENSIONS.join(", ")
        ));
    }
    let (sha256, size) = compute_sha256(path)?;
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let sha_match = match expected_sha256 {
        Some(expected) if !expected.is_empty() => {
            let ok = expected.eq_ignore_ascii_case(&sha256);
            if !ok {
                messages.push("SHA-256 does not match stored checksum".into());
            }
            ok
        }
        _ => true,
    };

    let valid = size > 0 && extension_allowed(path) && sha_match && messages.is_empty();
    if valid {
        messages.push("Artifact passed validation".into());
    }

    Ok(ValidationResult {
        valid,
        sha256,
        file_size: size,
        extension: ext,
        messages,
    })
}

pub fn validate_record(record: &FirmwareArtifactRecord) -> AppResult<ValidationResult> {
    let path = PathBuf::from(&record.file_path);
    validate_artifact_file(&path, Some(&record.sha256))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn rejects_empty_file() {
        let mut f = NamedTempFile::new().unwrap();
        f.write_all(b"").unwrap();
        let path = f.path().with_extension("bin");
        std::fs::copy(f.path(), &path).unwrap();
        assert!(compute_sha256(&path).is_err());
    }

    #[test]
    fn validates_bin_with_matching_hash() {
        let mut f = NamedTempFile::new().unwrap();
        f.write_all(b"firmware-bytes").unwrap();
        let path = f.path().with_extension("bin");
        std::fs::copy(f.path(), &path).unwrap();
        let (hash, _) = compute_sha256(&path).unwrap();
        let result = validate_artifact_file(&path, Some(&hash)).unwrap();
        assert!(result.valid);
    }
}
