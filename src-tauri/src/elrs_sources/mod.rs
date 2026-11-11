use crate::database::Database;
use crate::error::{AppError, AppResult};
use crate::firmware::compute_sha256;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const OFFICIAL_REPO: &str = "https://github.com/ExpressLRS/ExpressLRS";
pub const OFFICIAL_API_RELEASES: &str = "https://api.github.com/repos/ExpressLRS/ExpressLRS/releases";
pub const OFFICIAL_API_TAGS: &str = "https://api.github.com/repos/ExpressLRS/ExpressLRS/tags";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElrsReleaseInfo {
    pub tag_name: String,
    pub name: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
    pub published_at: Option<String>,
    pub html_url: Option<String>,
    pub tarball_url: Option<String>,
    pub zipball_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElrsTagInfo {
    pub name: String,
    pub commit_sha: Option<String>,
    pub zipball_url: Option<String>,
    pub tarball_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceDownloadRecord {
    pub id: String,
    pub project: String,
    pub version_or_ref: String,
    pub source_kind: String,
    pub remote_url: String,
    pub local_path: String,
    pub commit_sha: Option<String>,
    pub integrity_ok: bool,
    pub notes: Option<String>,
    pub downloaded_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchSourceRequest {
    pub version_or_ref: String,
    pub kind: String, // "tag" | "branch" | "release"
}

fn github_get(url: &str) -> AppResult<String> {
    let response = ureq::get(url)
        .set("User-Agent", "AetherFlashStudio/0.3")
        .set("Accept", "application/vnd.github+json")
        .call()
        .map_err(|e| AppError::Message(format!("GitHub API error: {e}")))?;
    response
        .into_string()
        .map_err(|e| AppError::Message(format!("GitHub response read error: {e}")))
}

pub fn list_official_releases() -> AppResult<Vec<ElrsReleaseInfo>> {
    let raw = github_get(OFFICIAL_API_RELEASES)?;
    let parsed: Vec<serde_json::Value> = serde_json::from_str(&raw)?;
    let mut out = Vec::new();
    for item in parsed {
        // Skip drafts; allow prerelease but mark it.
        if item.get("draft").and_then(|v| v.as_bool()).unwrap_or(false) {
            continue;
        }
        out.push(ElrsReleaseInfo {
            tag_name: item
                .get("tag_name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            name: item
                .get("name")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            draft: false,
            prerelease: item
                .get("prerelease")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            published_at: item
                .get("published_at")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            html_url: item
                .get("html_url")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            tarball_url: item
                .get("tarball_url")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            zipball_url: item
                .get("zipball_url")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
        });
    }
    Ok(out)
}

pub fn list_official_tags() -> AppResult<Vec<ElrsTagInfo>> {
    let raw = github_get(OFFICIAL_API_TAGS)?;
    let parsed: Vec<serde_json::Value> = serde_json::from_str(&raw)?;
    Ok(parsed
        .into_iter()
        .map(|item| ElrsTagInfo {
            name: item
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            commit_sha: item
                .pointer("/commit/sha")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            zipball_url: item
                .get("zipball_url")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            tarball_url: item
                .get("tarball_url")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
        })
        .collect())
}

fn git_available() -> bool {
    Command::new("git")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn run_git(args: &[&str], cwd: Option<&Path>) -> AppResult<String> {
    let mut cmd = Command::new("git");
    cmd.args(args);
    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }
    let output = cmd
        .output()
        .map_err(|e| AppError::Message(format!("Failed to run git: {e}")))?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::Message(format!("git failed: {}", err.trim())));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn sources_root(data_dir: &Path) -> PathBuf {
    data_dir.join("sources").join("ExpressLRS")
}

pub fn fetch_official_source(
    db: &Database,
    data_dir: &Path,
    request: FetchSourceRequest,
) -> AppResult<SourceDownloadRecord> {
    if !git_available() {
        return Err(AppError::Message(
            "git is required to download official ExpressLRS sources".into(),
        ));
    }

    // Only allow official remote.
    let remote = OFFICIAL_REPO.to_string();
    let safe_ref = sanitize_ref(&request.version_or_ref)?;
    let dest = sources_root(data_dir).join(&safe_ref);
    fs::create_dir_all(dest.parent().unwrap())?;

    if dest.join(".git").exists() {
        run_git(&["fetch", "--tags", "--force", "origin"], Some(&dest))?;
        run_git(
            &["checkout", "--force", request.version_or_ref.as_str()],
            Some(&dest),
        )?;
        let _ = run_git(&["pull", "--ff-only"], Some(&dest));
    } else {
        if dest.exists() {
            fs::remove_dir_all(&dest)?;
        }
        run_git(
            &[
                "clone",
                "--branch",
                request.version_or_ref.as_str(),
                "--single-branch",
                "--depth",
                "1",
                &remote,
                dest.to_str().unwrap_or("ExpressLRS"),
            ],
            None,
        )?;
    }

    let commit = run_git(&["rev-parse", "HEAD"], Some(&dest))?;
    let remote_url = run_git(&["remote", "get-url", "origin"], Some(&dest))?;
    if !remote_url.contains("ExpressLRS/ExpressLRS") {
        return Err(AppError::Message(
            "Refusing source tree: remote is not official ExpressLRS/ExpressLRS".into(),
        ));
    }

    let integrity_ok = !commit.is_empty() && dest.join("LICENSE").exists()
        || dest.join("src").exists()
        || dest.join("README.md").exists();

    // Marker file with integrity metadata
    let marker = dest.join(".aether_source_meta.json");
    let meta = serde_json::json!({
        "project": "ExpressLRS",
        "ref": request.version_or_ref,
        "commit": commit,
        "remote": OFFICIAL_REPO,
        "verifiedOfficialRemote": true,
        "updatedAt": Utc::now().to_rfc3339(),
    });
    fs::write(&marker, serde_json::to_string_pretty(&meta)?)?;
    let (_sha, _) = compute_sha256(&marker)?;

    let record = db.upsert_source_download(
        "ExpressLRS",
        &request.version_or_ref,
        &request.kind,
        OFFICIAL_REPO,
        &dest.display().to_string(),
        Some(&commit),
        integrity_ok,
        Some("Official ExpressLRS git checkout"),
    )?;

    let _ = db.record_operation(
        "elrs_source_fetch",
        None,
        "success",
        &format!("Fetched ExpressLRS {}", request.version_or_ref),
        None,
    );

    Ok(record)
}

fn sanitize_ref(value: &str) -> AppResult<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.contains("..") || trimmed.contains('\\') {
        return Err(AppError::Message("Invalid version/ref".into()));
    }
    let safe: String = trimmed
        .chars()
        .map(|c| if r#"<>:"|?*"#.contains(c) { '_' } else { c })
        .collect();
    Ok(safe.replace('/', "_"))
}

pub fn list_local_sources(db: &Database) -> AppResult<Vec<SourceDownloadRecord>> {
    db.list_source_downloads("ExpressLRS")
}

pub fn verify_local_source(path: &Path) -> AppResult<bool> {
    if !path.exists() {
        return Ok(false);
    }
    let remote = run_git(&["remote", "get-url", "origin"], Some(path)).unwrap_or_default();
    Ok(remote.contains("ExpressLRS/ExpressLRS"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_rejects_traversal() {
        assert!(sanitize_ref("../x").is_err());
        assert!(sanitize_ref("3.5.3").is_ok());
    }
}
