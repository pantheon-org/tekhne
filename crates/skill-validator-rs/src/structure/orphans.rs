//! Orphan-file detection (Go `structure/orphans.go`). Walks `assets/`,
//! `references/`, and `scripts/` and flags files never referenced (directly or
//! transitively) from SKILL.md via a BFS reachability walk. Path handling uses
//! forward-slash string semantics to match Go's `filepath.ToSlash` output; this
//! check is offline and deterministic.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

use super::fsutil::{is_text_file, read_dir_sorted, walk_files};
use crate::types::{ResultContext, ValidationResult};

/// Recognised subdirectories in a stable order for deterministic output
/// (Go `orderedRecognizedDirs`).
const ORDERED_RECOGNIZED_DIRS: &[&str] = &["assets", "references", "scripts"];

/// Python import statements: `from [.]*module import ...` or `import module`.
/// `\s`/`\w` are ASCII-scoped with `(?-u:...)` to match Go's RE2 semantics.
static PYTHON_IMPORT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?m)^(?-u:\s)*(?:from(?-u:\s)+((?-u:\.){0,2}(?-u:[\w.])+)(?-u:\s)+import|import(?-u:\s)+((?-u:[\w.])+))",
    )
    .expect("PYTHON_IMPORT_RE")
});

/// A text body queued for the BFS reachability walk.
struct QueueItem {
    text: String,
    /// The file that provided this text ("SKILL.md" for the seed).
    source: String,
}

/// Walk recognised directories to find files never referenced from SKILL.md
/// (Go `CheckOrphanFiles`).
pub fn check_orphan_files(dir: &Path, body: &str, opts: &super::Options) -> Vec<ValidationResult> {
    let ctx = ResultContext::new("Structure");
    let mut results: Vec<ValidationResult> = Vec::new();

    // Informational notes for allowed directories that exist on disk and are
    // not themselves recognised (those are covered by normal detection).
    for ad in &opts.allow_dirs {
        if ORDERED_RECOGNIZED_DIRS.contains(&ad.as_str()) {
            continue;
        }
        if dir.join(ad).exists() {
            results.push(ctx.info(format!(
                "{ad}/ skipped for orphan detection (allowed via --allow-dirs)"
            )));
        }
    }

    let inventory = inventory_files(dir);
    if inventory.is_empty() {
        return results;
    }

    let root_files = root_text_files(dir);

    let mut reached: BTreeSet<String> = BTreeSet::new();
    let mut reached_from: BTreeMap<String, String> = BTreeMap::new();
    let mut missing_extension: BTreeSet<String> = BTreeSet::new();
    let mut scanned_root_files: BTreeSet<String> = BTreeSet::new();
    let mut scanned_init_files: BTreeSet<String> = BTreeSet::new();

    let mut queue: Vec<QueueItem> = vec![QueueItem {
        text: body.to_string(),
        source: "SKILL.md".to_string(),
    }];

    while !queue.is_empty() {
        let item = queue.remove(0);

        // Directory of the source file (empty for the SKILL.md seed).
        let source_dir = if item.source == "SKILL.md" {
            String::new()
        } else {
            dir_of(&item.source)
        };

        // Pull in root-level files referenced by name (case-insensitive) so
        // they can bridge SKILL.md to files in recognised directories.
        let lower_text = item.text.to_lowercase();
        for rf in &root_files {
            if scanned_root_files.contains(rf) {
                continue;
            }
            if lower_text.contains(&rf.to_lowercase()) {
                scanned_root_files.insert(rf.clone());
                if let Ok(data) = std::fs::read_to_string(dir.join(rf)) {
                    queue.push(QueueItem {
                        text: data,
                        source: rf.clone(),
                    });
                }
            }
        }

        let is_python = item.source.ends_with(".py");

        let mut newly_reached: Vec<String> = Vec::new();
        for rel_path in &inventory {
            if reached.contains(rel_path) {
                continue;
            }
            if contains_reference(&item.text, &source_dir, rel_path) {
                newly_reached.push(rel_path.clone());
            } else if is_python && python_import_reaches(&item.text, &item.source, rel_path) {
                // Import resolution takes priority over the extensionless
                // fallback so normal imports do not trigger a "missing
                // extension" warning.
                newly_reached.push(rel_path.clone());
            } else if contains_reference_without_extension(&item.text, &source_dir, rel_path) {
                newly_reached.push(rel_path.clone());
                missing_extension.insert(rel_path.clone());
            }
        }
        for rel_path in newly_reached {
            mark_reached(
                &rel_path,
                &item.source,
                dir,
                &mut queue,
                &mut reached,
                &mut reached_from,
            );
        }

        // Python package `__init__.py` bridges.
        if is_python {
            for init_path in python_package_inits(&item.text, &item.source, dir) {
                if scanned_init_files.contains(&init_path) {
                    continue;
                }
                scanned_init_files.insert(init_path.clone());
                if let Ok(data) = std::fs::read_to_string(dir.join(&init_path)) {
                    queue.push(QueueItem {
                        text: data,
                        source: init_path,
                    });
                }
            }
        }
    }

    // Report per recognised directory.
    for d in ORDERED_RECOGNIZED_DIRS {
        let dir_files = files_in_dir(&inventory, d);
        if dir_files.is_empty() {
            continue;
        }

        let mut has_orphans = false;
        for rel_path in &dir_files {
            if !reached.contains(rel_path) {
                has_orphans = true;
                results.push(ctx.warn_file(
                    rel_path,
                    format!(
                        "potentially unreferenced file: {rel_path} — agents may not discover this \
                         file without an explicit reference in SKILL.md or a referenced file"
                    ),
                ));
            } else if missing_extension.contains(rel_path) {
                let ext = ext_of(rel_path);
                let no_ext = rel_path.strip_suffix(&ext).unwrap_or(rel_path);
                let from = reached_from.get(rel_path).cloned().unwrap_or_default();
                results.push(ctx.warn_file(
                    rel_path,
                    format!(
                        "file {rel_path} is referenced without its extension (as {no_ext} in \
                         {from}) — include the {ext} extension so agents can reliably locate the file"
                    ),
                ));
            }
        }

        if !has_orphans {
            results.push(ctx.pass(format!("all files in {d}/ are referenced")));
        }
    }

    results
}

/// Root-level text files excluding SKILL.md, used as BFS intermediaries
/// (Go `rootTextFiles`).
fn root_text_files(dir: &Path) -> Vec<String> {
    let mut files = Vec::new();
    for entry in read_dir_sorted(dir) {
        if entry.is_dir {
            continue;
        }
        if entry.name.eq_ignore_ascii_case("SKILL.md") {
            continue;
        }
        if is_text_file(&entry.name) {
            files.push(entry.name);
        }
    }
    files
}

/// Relative slash paths for all files under recognised directories, skipping
/// Python `__init__.py` markers (Go `inventoryFiles`).
fn inventory_files(dir: &Path) -> Vec<String> {
    let mut files = Vec::new();
    for d in ORDERED_RECOGNIZED_DIRS {
        let subdir = dir.join(d);
        let mut paths = Vec::new();
        walk_files(&subdir, &mut paths);
        for path in paths {
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            if name == "__init__.py" {
                continue;
            }
            let rel = path
                .strip_prefix(dir)
                .map(|p| p.to_string_lossy().replace(std::path::MAIN_SEPARATOR, "/"))
                .unwrap_or_else(|_| path.to_string_lossy().into_owned());
            files.push(rel);
        }
    }
    files
}

/// Inventory entries under the given directory prefix (Go `filesInDir`).
fn files_in_dir(inventory: &[String], dir: &str) -> Vec<String> {
    let prefix = format!("{dir}/");
    inventory
        .iter()
        .filter(|f| f.starts_with(&prefix))
        .cloned()
        .collect()
}

/// Whether `text` references `rel_path`, by full root-relative path or by a
/// path relative to `source_dir` (Go `containsReference`).
fn contains_reference(text: &str, source_dir: &str, rel_path: &str) -> bool {
    if text.contains(rel_path) {
        return true;
    }
    if !source_dir.is_empty() {
        if let Some(rel) = rel_to(source_dir, rel_path) {
            if !rel.starts_with("..") && text.contains(&rel) {
                return true;
            }
        }
    }
    false
}

/// Like [`contains_reference`] but strips the extension first
/// (Go `containsReferenceWithoutExtension`).
fn contains_reference_without_extension(text: &str, source_dir: &str, rel_path: &str) -> bool {
    let ext = ext_of(rel_path);
    if ext.is_empty() {
        return false;
    }
    let no_ext = rel_path.strip_suffix(&ext).unwrap_or(rel_path);
    contains_reference(text, source_dir, no_ext)
}

/// Mark a file reached and, if it is text, enqueue its contents
/// (Go `markReached`).
fn mark_reached(
    rel_path: &str,
    source: &str,
    dir: &Path,
    queue: &mut Vec<QueueItem>,
    reached: &mut BTreeSet<String>,
    reached_from: &mut BTreeMap<String, String>,
) {
    reached.insert(rel_path.to_string());
    reached_from.insert(rel_path.to_string(), source.to_string());

    if is_text_file(rel_path) {
        if let Ok(data) = std::fs::read_to_string(dir.join(rel_path)) {
            queue.push(QueueItem {
                text: data,
                source: rel_path.to_string(),
            });
        }
    }
}

/// Whether any Python import in `text` resolves to `rel_path`
/// (Go `pythonImportReaches`).
fn python_import_reaches(text: &str, source: &str, rel_path: &str) -> bool {
    if !rel_path.ends_with(".py") {
        return false;
    }
    let source_dir = dir_of(source);

    for module in import_modules(text) {
        let (module, resolve_dir) = resolve_relative(&module, &source_dir);
        if module.is_empty() {
            continue;
        }
        let module_path = module.replace('.', "/");
        let candidate = join_slash(&resolve_dir, &format!("{module_path}.py"));
        if candidate == rel_path {
            return true;
        }
    }
    false
}

/// `__init__.py` paths for Python imports that resolve to package directories
/// present on disk (Go `pythonPackageInits`).
fn python_package_inits(text: &str, source: &str, dir: &Path) -> Vec<String> {
    let source_dir = dir_of(source);
    let mut inits = Vec::new();

    for module in import_modules(text) {
        let (module, resolve_dir) = resolve_relative(&module, &source_dir);
        if module.is_empty() {
            continue;
        }
        let module_path = module.replace('.', "/");
        let init_path = join_slash(&resolve_dir, &format!("{module_path}/__init__.py"));
        if dir.join(&init_path).exists() {
            inits.push(init_path);
        }
    }
    inits
}

/// Root-level, non-SKILL.md files unreferenced in the body (Go
/// `CheckFlatOrphanFiles`).
pub fn check_flat_orphan_files(dir: &Path, body: &str) -> Vec<ValidationResult> {
    let ctx = ResultContext::new("Structure");

    let mut root_files: Vec<String> = Vec::new();
    for entry in read_dir_sorted(dir) {
        if entry.is_dir || entry.name.starts_with('.') {
            continue;
        }
        if entry.name.eq_ignore_ascii_case("SKILL.md") {
            continue;
        }
        root_files.push(entry.name);
    }

    if root_files.is_empty() {
        return Vec::new();
    }

    let mut results = Vec::new();
    let mut has_orphans = false;
    for name in &root_files {
        if !contains_reference(body, "", name)
            && !contains_reference_without_extension(body, "", name)
        {
            has_orphans = true;
            results.push(ctx.warn_file(
                name,
                format!(
                    "potentially unreferenced file: {name} — agents may not discover this file \
                     without an explicit reference in SKILL.md"
                ),
            ));
        }
    }

    if !has_orphans {
        results.push(ctx.pass("all root-level files are referenced from SKILL.md"));
    }

    results
}

/// Collect the module strings from Python import statements in `text`.
fn import_modules(text: &str) -> Vec<String> {
    PYTHON_IMPORT_RE
        .captures_iter(text)
        .filter_map(|caps| {
            caps.get(1)
                .or_else(|| caps.get(2))
                .map(|m| m.as_str().to_string())
        })
        .collect()
}

/// Resolve a possibly relative import module against `source_dir`, returning
/// the bare module and the directory to resolve it in (Go's leading-dot logic).
fn resolve_relative(module: &str, source_dir: &str) -> (String, String) {
    let mut module = module.to_string();
    let mut resolve_dir = source_dir.to_string();
    if module.starts_with('.') {
        module = module[1..].to_string(); // first dot: current package
        while module.starts_with('.') {
            module = module[1..].to_string();
            resolve_dir = dir_of(&resolve_dir);
        }
    }
    (module, resolve_dir)
}

/// Go `filepath.Dir` for a forward-slash path.
fn dir_of(path: &str) -> String {
    match path.rfind('/') {
        Some(0) => "/".to_string(),
        Some(i) => path[..i].to_string(),
        None => ".".to_string(),
    }
}

/// Go `filepath.Ext`: the extension including the dot, or empty.
fn ext_of(path: &str) -> String {
    let base = path.rsplit('/').next().unwrap_or(path);
    match base.rfind('.') {
        Some(i) => base[i..].to_string(),
        None => String::new(),
    }
}

/// Go `filepath.Join(a, b)` then `ToSlash`, for relative slash paths.
fn join_slash(a: &str, b: &str) -> String {
    if a.is_empty() || a == "." {
        return clean_slash(b);
    }
    if b.is_empty() {
        return clean_slash(a);
    }
    clean_slash(&format!("{a}/{b}"))
}

/// Lexical `filepath.Clean` for relative forward-slash paths.
fn clean_slash(path: &str) -> String {
    if path.is_empty() {
        return ".".to_string();
    }
    let mut out: Vec<&str> = Vec::new();
    for seg in path.split('/') {
        match seg {
            "" | "." => continue,
            ".." => {
                if matches!(out.last(), Some(&last) if last != "..") {
                    out.pop();
                } else {
                    out.push("..");
                }
            }
            _ => out.push(seg),
        }
    }
    if out.is_empty() {
        ".".to_string()
    } else {
        out.join("/")
    }
}

/// Go `filepath.Rel(base, target)` for relative forward-slash paths. Returns
/// `None` when a relative path cannot be computed.
fn rel_to(base: &str, target: &str) -> Option<String> {
    let base = clean_slash(base);
    let target = clean_slash(target);
    if base == target {
        return Some(".".to_string());
    }
    let base_segs: Vec<&str> = if base == "." {
        Vec::new()
    } else {
        base.split('/').collect()
    };
    let targ_segs: Vec<&str> = if target == "." {
        Vec::new()
    } else {
        target.split('/').collect()
    };

    let mut i = 0;
    while i < base_segs.len() && i < targ_segs.len() && base_segs[i] == targ_segs[i] {
        i += 1;
    }

    let mut out: Vec<String> = Vec::new();
    for _ in i..base_segs.len() {
        out.push("..".to_string());
    }
    out.extend(targ_segs[i..].iter().map(|s| (*s).to_string()));

    if out.is_empty() {
        Some(".".to_string())
    } else {
        Some(out.join("/"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn write(dir: &Path, rel: &str, contents: &str) {
        let path = dir.join(rel);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, contents).unwrap();
    }

    #[test]
    fn dir_and_ext_helpers() {
        assert_eq!(dir_of("references/guide.md"), "references");
        assert_eq!(dir_of("guide.md"), ".");
        assert_eq!(ext_of("scripts/run.py"), ".py");
        assert_eq!(ext_of("Makefile"), "");
    }

    #[test]
    fn rel_and_join_helpers() {
        assert_eq!(
            rel_to("references", "references/img/x.png").unwrap(),
            "img/x.png"
        );
        assert_eq!(
            join_slash("references", "img/x.png"),
            "references/img/x.png"
        );
        assert_eq!(join_slash(".", "helpers.py"), "helpers.py");
    }

    #[test]
    fn referenced_and_orphan_files() {
        let tmp = TempDir::new().unwrap();
        let body = "See [guide](references/guide.md) for details.";
        write(tmp.path(), "SKILL.md", body);
        write(tmp.path(), "references/guide.md", "# Guide\n");
        write(tmp.path(), "references/orphan.md", "# Orphan\n");

        let results = check_orphan_files(tmp.path(), body, &super::super::Options::default());
        assert!(results.iter().any(|r| r.file == "references/orphan.md"
            && r.message.contains("potentially unreferenced file")));
        assert!(!results
            .iter()
            .any(|r| r.file == "references/guide.md"
                && r.message.contains("potentially unreferenced")));
    }

    #[test]
    fn transitive_reference_through_reference_file() {
        let tmp = TempDir::new().unwrap();
        let body = "Start with [a](references/a.md).";
        write(tmp.path(), "SKILL.md", body);
        // a.md references b.md relative to its own directory.
        write(tmp.path(), "references/a.md", "Then read [b](b.md).\n");
        write(tmp.path(), "references/b.md", "# B\n");

        let results = check_orphan_files(tmp.path(), body, &super::super::Options::default());
        assert!(!results
            .iter()
            .any(|r| r.message.contains("potentially unreferenced")));
    }

    #[test]
    fn python_import_reaches_module() {
        let tmp = TempDir::new().unwrap();
        let body = "Run [main](scripts/main.py).";
        write(tmp.path(), "SKILL.md", body);
        write(tmp.path(), "scripts/main.py", "from helpers import merge\n");
        write(tmp.path(), "scripts/helpers.py", "def merge():\n    pass\n");

        let results = check_orphan_files(tmp.path(), body, &super::super::Options::default());
        assert!(!results
            .iter()
            .any(|r| r.message.contains("potentially unreferenced")));
    }
}
