//! Shared skill-listing budget report.
//!
//! Claude Code puts every listing-eligible skill's name and description into
//! context each turn. Besides the per-entry cap, the entries share one
//! aggregate budget (`skillListingBudgetFraction`, 1% of the context window;
//! the documented fallback is 8000 characters). When it overflows, descriptions
//! of the least-invoked skills are dropped, with no local signal. This module
//! estimates that aggregate statically. The result is advisory: the live budget
//! depends on the consumer's model and settings, which a repository cannot see.
//!
//! Sources: <https://code.claude.com/docs/en/skills> (skill descriptions are
//! cut short) and <https://code.claude.com/docs/en/settings>.

use std::collections::BTreeMap;
use std::path::Path;

use serde::Serialize;
use serde_yaml_ng::Value;

use crate::skill::Skill;
use crate::structure::fsutil::read_dir_sorted;

/// Documented fallback for the shared budget, in characters.
pub const DEFAULT_BUDGET_CHARS: usize = 8000;
/// Documented per-entry cap (`skillListingMaxDescChars`), in characters.
pub const MAX_DESC_CHARS: usize = 1536;
/// Joiner between a skill name and its description in the listing.
const JOINER: &str = " - ";

/// One skill's contribution to the listing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ListingEntry {
    /// Skill directory relative to the scanned root.
    pub path: String,
    /// Effective skill name (declared `name`, else the directory name).
    pub name: String,
    /// Characters this skill spends: name + joiner + capped description.
    pub chars: usize,
    /// True when the description text was cut at the per-entry cap.
    pub truncated: bool,
}

/// The aggregate report over one skills root.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListingReport {
    /// The scanned root, as supplied.
    pub root: String,
    /// The budget the total is compared against, in characters.
    pub budget_chars: usize,
    /// Sum of `chars` over all counted skills.
    pub total_chars: usize,
    /// Listing-eligible skills, largest first.
    pub entries: Vec<ListingEntry>,
    /// Skills skipped because `disable-model-invocation` is true.
    pub excluded_disabled: usize,
    /// Skills whose SKILL.md could not be parsed (not counted).
    pub unreadable: Vec<String>,
    /// Character totals per top-level domain directory.
    pub by_domain: BTreeMap<String, usize>,
}

impl ListingReport {
    /// Whether the estimated aggregate exceeds the budget.
    pub fn over_budget(&self) -> bool {
        self.total_chars > self.budget_chars
    }
}

/// Scan `root` for skills and estimate the shared listing cost against
/// `budget_chars`.
pub fn analyze(root: &Path, budget_chars: usize) -> ListingReport {
    let mut report = ListingReport {
        root: root.to_string_lossy().into_owned(),
        budget_chars,
        ..ListingReport::default()
    };

    let mut dirs = Vec::new();
    find_skill_dirs(root, &mut dirs);

    for dir in dirs {
        let rel = dir
            .strip_prefix(root)
            .unwrap_or(&dir)
            .to_string_lossy()
            .replace('\\', "/");
        let skill = match Skill::load(&dir) {
            Ok(s) => s,
            Err(_) => {
                report.unreadable.push(rel);
                continue;
            }
        };
        if is_disabled(&skill.raw_frontmatter) {
            report.excluded_disabled += 1;
            continue;
        }
        let entry = entry_for(&skill, &dir, rel);
        let domain = entry.path.split('/').next().unwrap_or("").to_string();
        *report.by_domain.entry(domain).or_insert(0) += entry.chars;
        report.total_chars += entry.chars;
        report.entries.push(entry);
    }

    report
        .entries
        .sort_by(|a, b| b.chars.cmp(&a.chars).then_with(|| a.path.cmp(&b.path)));
    report
}

fn entry_for(skill: &Skill, dir: &Path, path: String) -> ListingEntry {
    let name = if skill.frontmatter.name.is_empty() {
        dir.file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default()
    } else {
        skill.frontmatter.name.clone()
    };

    let mut text = skill.frontmatter.description.trim().to_string();
    if let Some(when) = string_field(&skill.raw_frontmatter, "when_to_use") {
        if !when.trim().is_empty() {
            text.push(' ');
            text.push_str(when.trim());
        }
    }
    let text_chars = text.chars().count();
    let truncated = text_chars > MAX_DESC_CHARS;
    let desc_chars = text_chars.min(MAX_DESC_CHARS);

    ListingEntry {
        path,
        chars: name.chars().count() + JOINER.len() + desc_chars,
        name,
        truncated,
    }
}

fn string_field(raw: &Value, key: &str) -> Option<String> {
    match raw.get(key)? {
        Value::String(s) => Some(s.clone()),
        _ => None,
    }
}

/// `disable-model-invocation: true` removes the description from the listing.
fn is_disabled(raw: &Value) -> bool {
    match raw.get("disable-model-invocation") {
        Some(Value::Bool(b)) => *b,
        Some(Value::String(s)) => s.eq_ignore_ascii_case("true"),
        _ => false,
    }
}

/// Collect directories holding a `SKILL.md`, sorted. Hidden directories,
/// `node_modules` and `target` are skipped, and a skill's own subtree is not
/// searched (its fixtures and references are not skills).
fn find_skill_dirs(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    if dir.join("SKILL.md").is_file() {
        out.push(dir.to_path_buf());
        return;
    }
    for entry in read_dir_sorted(dir) {
        if !entry.is_dir
            || entry.name.starts_with('.')
            || entry.name == "node_modules"
            || entry.name == "target"
        {
            continue;
        }
        find_skill_dirs(&entry.path, out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn write_skill(root: &Path, rel: &str, frontmatter: &str) {
        let dir = root.join(rel);
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("SKILL.md"),
            format!("---\n{frontmatter}---\n\nBody\n"),
        )
        .unwrap();
    }

    #[test]
    fn counts_name_joiner_and_description() {
        let tmp = tempfile::tempdir().unwrap();
        write_skill(
            tmp.path(),
            "dev/alpha",
            "name: alpha\ndescription: Does a thing\n",
        );
        let r = analyze(tmp.path(), 100);
        assert_eq!(r.entries.len(), 1);
        assert_eq!(r.total_chars, "alpha".len() + 3 + "Does a thing".len());
        assert_eq!(r.by_domain["dev"], r.total_chars);
        assert!(!r.over_budget());
    }

    #[test]
    fn skips_disable_model_invocation() {
        let tmp = tempfile::tempdir().unwrap();
        write_skill(
            tmp.path(),
            "dev/hidden",
            "name: hidden\ndescription: x\ndisable-model-invocation: true\n",
        );
        write_skill(tmp.path(), "dev/shown", "name: shown\ndescription: x\n");
        let r = analyze(tmp.path(), 100);
        assert_eq!(r.excluded_disabled, 1);
        assert_eq!(r.entries.len(), 1);
        assert_eq!(r.entries[0].name, "shown");
    }

    #[test]
    fn caps_description_and_adds_when_to_use() {
        let tmp = tempfile::tempdir().unwrap();
        let long = "a".repeat(2000);
        write_skill(
            tmp.path(),
            "dev/long",
            &format!("name: long\ndescription: {long}\nwhen_to_use: extra\n"),
        );
        let r = analyze(tmp.path(), 100);
        assert!(r.entries[0].truncated);
        assert_eq!(r.entries[0].chars, 4 + 3 + MAX_DESC_CHARS);
        assert!(r.over_budget());
    }

    #[test]
    fn falls_back_to_directory_name_and_orders_largest_first() {
        let tmp = tempfile::tempdir().unwrap();
        write_skill(tmp.path(), "dev/small", "description: s\n");
        write_skill(
            tmp.path(),
            "ops/big",
            "name: big\ndescription: much longer text\n",
        );
        let r = analyze(tmp.path(), 8000);
        assert_eq!(r.entries[0].name, "big");
        assert_eq!(r.entries[1].name, "small");
    }

    #[test]
    fn does_not_descend_into_a_skill() {
        let tmp = tempfile::tempdir().unwrap();
        write_skill(tmp.path(), "dev/outer", "name: outer\ndescription: x\n");
        write_skill(
            tmp.path(),
            "dev/outer/evals/fixture",
            "name: fixture\ndescription: x\n",
        );
        let r = analyze(tmp.path(), 8000);
        assert_eq!(r.entries.len(), 1);
    }

    #[test]
    fn reports_unparseable_skill() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("dev/bad");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("SKILL.md"), "---\nname: bad\nnever closed\n").unwrap();
        let r = analyze(tmp.path(), 8000);
        assert_eq!(r.unreadable, vec!["dev/bad".to_string()]);
    }
}
