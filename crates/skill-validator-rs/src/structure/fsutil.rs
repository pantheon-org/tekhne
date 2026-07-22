//! Shared filesystem helpers for the structure checks. Iteration is sorted to
//! match Go's `os.ReadDir` / `filepath.Walk` lexical ordering, so token counts
//! and orphan walks are deterministic.

use std::path::{Path, PathBuf};

const BINARY_EXTENSIONS: &[&str] = &[
    ".png", ".jpg", ".jpeg", ".gif", ".bmp", ".ico", ".svg", ".webp", ".pdf", ".doc", ".docx",
    ".xls", ".xlsx", ".zip", ".tar", ".gz", ".bz2", ".7z", ".rar", ".exe", ".dll", ".so", ".dylib",
    ".bin", ".mp3", ".mp4", ".wav", ".avi", ".mov", ".woff", ".woff2", ".ttf", ".eot", ".otf",
];

/// A directory entry: name, full path, and directory flag.
pub struct Entry {
    /// The base name of the entry.
    pub name: String,
    /// The full path to the entry.
    pub path: PathBuf,
    /// Whether the entry is a directory.
    pub is_dir: bool,
}

/// Read a directory's entries sorted lexically by name (Go `os.ReadDir`).
/// Returns an empty vector when the directory cannot be read.
pub fn read_dir_sorted(dir: &Path) -> Vec<Entry> {
    let mut entries = Vec::new();
    let Ok(rd) = std::fs::read_dir(dir) else {
        return entries;
    };
    for e in rd.flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        let path = e.path();
        let is_dir = path.is_dir();
        entries.push(Entry { name, path, is_dir });
    }
    entries.sort_by(|a, b| a.name.cmp(&b.name));
    entries
}

/// Walk files under `root` in Go `filepath.Walk` order (lexical, pre-order),
/// skipping hidden files and hidden subdirectories.
pub fn walk_files(root: &Path, out: &mut Vec<PathBuf>) {
    for e in read_dir_sorted(root) {
        if e.name.starts_with('.') {
            continue;
        }
        if e.is_dir {
            walk_files(&e.path, out);
        } else {
            out.push(e.path);
        }
    }
}

/// The lower-cased extension including the dot (Go `filepath.Ext` + `ToLower`).
pub fn ext_lower(name: &str) -> String {
    match name.rfind('.') {
        Some(i) => name[i..].to_lowercase(),
        None => String::new(),
    }
}

/// Whether `name`'s extension is in the binary skip-list.
pub fn is_binary_ext(name: &str) -> bool {
    BINARY_EXTENSIONS.contains(&ext_lower(name).as_str())
}

/// A file is "text" when its extension is not in the binary list
/// (Go `structure.isTextFile`).
pub fn is_text_file(name: &str) -> bool {
    !is_binary_ext(name)
}

/// Relative path from `base` to `path`, using forward slashes
/// (Go `filepath.Rel` + `filepath.ToSlash`).
pub fn rel_slash(base: &Path, path: &Path) -> String {
    path.strip_prefix(base)
        .map(|p| p.to_string_lossy().replace(std::path::MAIN_SEPARATOR, "/"))
        .unwrap_or_else(|_| path.to_string_lossy().into_owned())
}

/// Read a file to a string, or `None` on error.
pub fn read_file(path: &Path) -> Option<String> {
    std::fs::read_to_string(path).ok()
}
