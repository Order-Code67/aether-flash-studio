use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceNode {
    pub path: String,
    pub name: String,
    pub is_dir: bool,
    pub size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileContent {
    pub path: String,
    pub content: String,
    pub language: String,
    pub size: u64,
    pub readonly: bool,
}

fn ensure_inside(root: &Path, candidate: &Path) -> AppResult<PathBuf> {
    let root = fs::canonicalize(root).map_err(|e| AppError::Message(e.to_string()))?;
    let full = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        root.join(candidate)
    };
    let canon = fs::canonicalize(&full).unwrap_or(full.clone());
    if !canon.starts_with(&root) {
        return Err(AppError::Message(
            "Path escapes workspace root — blocked".into(),
        ));
    }
    Ok(canon)
}

pub fn list_tree(root: &Path, max_entries: usize) -> AppResult<Vec<WorkspaceNode>> {
    if !root.exists() {
        return Err(AppError::Message("Workspace root does not exist".into()));
    }
    let mut out = Vec::new();
    for entry in WalkDir::new(root)
        .max_depth(8)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if out.len() >= max_entries {
            break;
        }
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if name == ".git" || name == "node_modules" || name == ".pio" {
            continue;
        }
        let rel = path.strip_prefix(root).unwrap_or(path);
        out.push(WorkspaceNode {
            path: rel.to_string_lossy().replace('\\', "/"),
            name,
            is_dir: path.is_dir(),
            size: if path.is_file() {
                fs::metadata(path).ok().map(|m| m.len())
            } else {
                None
            },
        });
    }
    out.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(out)
}

pub fn read_file(root: &Path, relative: &str) -> AppResult<FileContent> {
    let path = ensure_inside(root, Path::new(relative))?;
    if !path.is_file() {
        return Err(AppError::Message("Not a file".into()));
    }
    let meta = fs::metadata(&path)?;
    if meta.len() > 2_000_000 {
        return Err(AppError::Message(
            "File too large for editor (>2MB)".into(),
        ));
    }
    let content = fs::read_to_string(&path)
        .map_err(|_| AppError::Message("Binary or non-UTF8 file — open blocked".into()))?;
    let language = match path.extension().and_then(|e| e.to_str()).unwrap_or("") {
        "rs" => "rust",
        "py" => "python",
        "json" => "json",
        "md" => "markdown",
        "ini" | "cfg" => "ini",
        "yml" | "yaml" => "yaml",
        "c" | "h" | "cpp" | "hpp" => "cpp",
        "js" | "ts" => "typescript",
        _ => "plaintext",
    };
    Ok(FileContent {
        path: relative.replace('\\', "/"),
        content,
        language: language.into(),
        size: meta.len(),
        readonly: false,
    })
}

pub fn write_file(root: &Path, relative: &str, content: &str) -> AppResult<()> {
    let path = ensure_inside(root, Path::new(relative))?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    // Keep original for undo sidecar if missing
    let bak = path.with_extension(format!(
        "{}.aetherbak",
        path.extension().and_then(|e| e.to_str()).unwrap_or("txt")
    ));
    if path.exists() && !bak.exists() {
        let _ = fs::copy(&path, &bak);
    }
    fs::write(&path, content)?;
    Ok(())
}

pub fn revert_file(root: &Path, relative: &str) -> AppResult<()> {
    let path = ensure_inside(root, Path::new(relative))?;
    let bak = path.with_extension(format!(
        "{}.aetherbak",
        path.extension().and_then(|e| e.to_str()).unwrap_or("txt")
    ));
    if !bak.exists() {
        return Err(AppError::Message("No backup snapshot for revert".into()));
    }
    fs::copy(&bak, &path)?;
    Ok(())
}

pub fn search_project(root: &Path, query: &str, max_hits: usize) -> AppResult<Vec<String>> {
    if query.trim().len() < 2 {
        return Ok(Vec::new());
    }
    let mut hits = Vec::new();
    for entry in WalkDir::new(root).max_depth(8).into_iter().filter_map(|e| e.ok()) {
        if hits.len() >= max_hits {
            break;
        }
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if let Ok(text) = fs::read_to_string(path) {
            if text.contains(query) {
                let rel = path.strip_prefix(root).unwrap_or(path);
                hits.push(rel.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    Ok(hits)
}

pub fn diff_against_backup(root: &Path, relative: &str) -> AppResult<String> {
    let path = ensure_inside(root, Path::new(relative))?;
    let bak = path.with_extension(format!(
        "{}.aetherbak",
        path.extension().and_then(|e| e.to_str()).unwrap_or("txt")
    ));
    if !bak.exists() {
        return Ok("No baseline backup yet.".into());
    }
    let a = fs::read_to_string(&bak).unwrap_or_default();
    let b = fs::read_to_string(&path).unwrap_or_default();
    if a == b {
        return Ok("No changes.".into());
    }
    Ok(format!(
        "Baseline bytes: {}\nCurrent bytes: {}\nChanged: true\n(Full line-diff UI on frontend)",
        a.len(),
        b.len()
    ))
}
