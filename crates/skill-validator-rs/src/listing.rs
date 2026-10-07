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
/// Documented default for `skillListingBudgetFraction`.
pub const DEFAULT_FRACTION: f64 = 0.01;
/// Assumed characters per token when deriving a budget from a context window.
/// This is an assumption of the estimate, not a documented value.
pub const DEFAULT_CHARS_PER_TOKEN: f64 = 4.0;
/// Context windows reported as a band when only a fraction is known. Also an
/// assumption: the window is per model and cannot be read from disk.
pub const BAND_WINDOWS: [usize; 2] = [200_000, 1_000_000];
/// Joiner between a skill name and its description in the listing.
const JOINER: &str = " - ";

/// What the caller knows about the budget. Every field is optional; see
/// [`resolve_budget`] for precedence.
#[derive(Debug, Clone, Default)]
pub struct BudgetInputs {
    /// A fixed aggregate budget in characters.
    pub chars: Option<usize>,
    /// The model's context window in tokens.
    pub context_tokens: Option<usize>,
    /// `skillListingBudgetFraction`.
    pub fraction: Option<f64>,
    /// Characters per token (default [`DEFAULT_CHARS_PER_TOKEN`]).
    pub chars_per_token: Option<f64>,
}

/// One row of the context-window band.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BudgetBand {
    /// The assumed context window in tokens.
    pub context_tokens: usize,
    /// The budget that window gives, in characters.
    pub budget_chars: usize,
    /// Whether the estimated total exceeds it.
    pub over: bool,
}

/// A resolved budget and how it was derived.
#[derive(Debug, Clone, PartialEq)]
pub struct Budget {
    /// The budget used for the headline verdict, in characters.
    pub chars: usize,
    /// Human-readable derivation.
    pub source: String,
    /// Band windows, set only when a fraction is known without a window.
    pub band_fraction: Option<f64>,
    /// Characters per token used for any derivation.
    pub chars_per_token: f64,
}

fn derive_chars(tokens: usize, chars_per_token: f64, fraction: f64) -> usize {
    (tokens as f64 * chars_per_token * fraction).floor() as usize
}

/// Resolve the budget. Precedence, highest first:
/// 1. `chars` (a fixed budget);
/// 2. `context_tokens` x `chars_per_token` x `fraction` (default fraction);
/// 3. a `fraction` alone, reported as a band over [`BAND_WINDOWS`] with the
///    smallest window as the headline;
/// 4. the documented [`DEFAULT_BUDGET_CHARS`] fallback.
pub fn resolve_budget(inputs: &BudgetInputs) -> Budget {
    let cpt = inputs.chars_per_token.unwrap_or(DEFAULT_CHARS_PER_TOKEN);
    if let Some(chars) = inputs.chars {
        return Budget {
            chars,
            source: "fixed character budget".to_string(),
            band_fraction: None,
            chars_per_token: cpt,
        };
    }
    if let Some(tokens) = inputs.context_tokens {
        let fraction = inputs.fraction.unwrap_or(DEFAULT_FRACTION);
        return Budget {
            chars: derive_chars(tokens, cpt, fraction),
            source: format!("{tokens} tokens x {cpt} chars/token x {fraction}"),
            band_fraction: None,
            chars_per_token: cpt,
        };
    }
    if let Some(fraction) = inputs.fraction {
        return Budget {
            chars: derive_chars(BAND_WINDOWS[0], cpt, fraction),
            source: format!("fraction {fraction}, band over assumed context windows"),
            band_fraction: Some(fraction),
            chars_per_token: cpt,
        };
    }
    Budget {
        chars: DEFAULT_BUDGET_CHARS,
        source: "documented fallback (SLASH_COMMAND_TOOL_CHAR_BUDGET schema)".to_string(),
        band_fraction: None,
        chars_per_token: cpt,
    }
}

/// Values read from the machine's environment and settings files.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MachineSettings {
    /// `SLASH_COMMAND_TOOL_CHAR_BUDGET`, when set to a positive integer.
    pub env_chars: Option<usize>,
    /// `skillListingBudgetFraction` from the first settings file that sets it.
    pub fraction: Option<f64>,
    /// What was read and what was not, for the report.
    pub notes: Vec<String>,
}

/// Read the budget this machine resolves. `env` looks up an environment
/// variable; `cwd` seeds the project-root search. Managed policy settings and a
/// settings `env` block are not read, and the notes say so.
pub fn read_machine_settings(env: &dyn Fn(&str) -> Option<String>, cwd: &Path) -> MachineSettings {
    let mut out = MachineSettings::default();

    if let Some(raw) = env("SLASH_COMMAND_TOOL_CHAR_BUDGET") {
        match raw.trim().parse::<usize>() {
            Ok(n) if n > 0 => {
                out.env_chars = Some(n);
                out.notes.push(format!(
                    "SLASH_COMMAND_TOOL_CHAR_BUDGET={n} from the environment"
                ));
            }
            _ => out.notes.push(format!(
                "SLASH_COMMAND_TOOL_CHAR_BUDGET={raw:?} ignored (not a positive integer)"
            )),
        }
    }

    let project = env("CLAUDE_PROJECT_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| project_root(cwd));
    let config = env("CLAUDE_CONFIG_DIR")
        .map(std::path::PathBuf::from)
        .or_else(|| env("HOME").map(|h| Path::new(&h).join(".claude")));

    let mut files = vec![
        project.join(".claude/settings.local.json"),
        project.join(".claude/settings.json"),
    ];
    if let Some(c) = config {
        files.push(c.join("settings.json"));
    }
    for file in files {
        let Ok(text) = std::fs::read_to_string(&file) else {
            continue;
        };
        let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) else {
            out.notes
                .push(format!("{} is not valid JSON, skipped", file.display()));
            continue;
        };
        if let Some(f) = json
            .get("skillListingBudgetFraction")
            .and_then(|v| v.as_f64())
            .filter(|f| *f > 0.0 && *f <= 1.0)
        {
            out.fraction = Some(f);
            out.notes.push(format!(
                "skillListingBudgetFraction={f} from {}",
                file.display()
            ));
            break;
        }
    }

    out.notes
        .push("managed policy settings and the settings env block are not read".to_string());
    out
}

/// The nearest ancestor of `cwd` holding a `.git`, else `cwd` itself.
fn project_root(cwd: &Path) -> std::path::PathBuf {
    cwd.ancestors()
        .find(|a| a.join(".git").exists())
        .unwrap_or(cwd)
        .to_path_buf()
}

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
    /// How the budget was derived.
    pub budget_source: String,
    /// The same total against each assumed context window; empty unless only a
    /// fraction was known.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bands: Vec<BudgetBand>,
    /// Notes about the inputs (settings files read, anything skipped).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
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
/// `budget`.
pub fn analyze(root: &Path, budget: &Budget) -> ListingReport {
    let mut report = ListingReport {
        root: root.to_string_lossy().into_owned(),
        budget_chars: budget.chars,
        budget_source: budget.source.clone(),
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

    if let Some(fraction) = budget.band_fraction {
        report.bands = BAND_WINDOWS
            .iter()
            .map(|&tokens| {
                let chars = derive_chars(tokens, budget.chars_per_token, fraction);
                BudgetBand {
                    context_tokens: tokens,
                    budget_chars: chars,
                    over: report.total_chars > chars,
                }
            })
            .collect();
    }
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

    fn fixed(chars: usize) -> Budget {
        resolve_budget(&BudgetInputs {
            chars: Some(chars),
            ..BudgetInputs::default()
        })
    }

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
        let r = analyze(tmp.path(), &fixed(100));
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
        let r = analyze(tmp.path(), &fixed(100));
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
        let r = analyze(tmp.path(), &fixed(100));
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
        let r = analyze(tmp.path(), &fixed(8000));
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
        let r = analyze(tmp.path(), &fixed(8000));
        assert_eq!(r.entries.len(), 1);
    }

    #[test]
    fn reports_unparseable_skill() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("dev/bad");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("SKILL.md"), "---\nname: bad\nnever closed\n").unwrap();
        let r = analyze(tmp.path(), &fixed(8000));
        assert_eq!(r.unreadable, vec!["dev/bad".to_string()]);
    }

    #[test]
    fn budget_precedence() {
        let d = resolve_budget(&BudgetInputs::default());
        assert_eq!(d.chars, DEFAULT_BUDGET_CHARS);

        let tokens = resolve_budget(&BudgetInputs {
            context_tokens: Some(1_000_000),
            ..BudgetInputs::default()
        });
        assert_eq!(tokens.chars, 40_000);

        let fixed_wins = resolve_budget(&BudgetInputs {
            chars: Some(123),
            context_tokens: Some(1_000_000),
            fraction: Some(0.5),
            ..BudgetInputs::default()
        });
        assert_eq!(fixed_wins.chars, 123);

        let frac = resolve_budget(&BudgetInputs {
            fraction: Some(0.02),
            ..BudgetInputs::default()
        });
        assert_eq!(frac.chars, 16_000);
        assert_eq!(frac.band_fraction, Some(0.02));
    }

    #[test]
    fn fraction_only_reports_a_band() {
        let tmp = tempfile::tempdir().unwrap();
        write_skill(tmp.path(), "dev/a", "name: a\ndescription: x\n");
        let b = resolve_budget(&BudgetInputs {
            fraction: Some(0.000001),
            ..BudgetInputs::default()
        });
        let r = analyze(tmp.path(), &b);
        assert_eq!(r.bands.len(), 2);
        assert_eq!(r.bands[0].context_tokens, 200_000);
        assert!(r.bands.iter().all(|x| x.over));
    }

    #[test]
    fn machine_settings_reads_first_file_that_sets_the_fraction() {
        let tmp = tempfile::tempdir().unwrap();
        let project = tmp.path().join("proj");
        let config = tmp.path().join("cfg");
        fs::create_dir_all(project.join(".claude")).unwrap();
        fs::create_dir_all(&config).unwrap();
        fs::write(
            project.join(".claude/settings.json"),
            r#"{"skillListingBudgetFraction": 0.02}"#,
        )
        .unwrap();
        fs::write(
            config.join("settings.json"),
            r#"{"skillListingBudgetFraction": 0.5}"#,
        )
        .unwrap();
        let project_s = project.to_string_lossy().into_owned();
        let config_s = config.to_string_lossy().into_owned();
        let env = move |k: &str| match k {
            "CLAUDE_PROJECT_DIR" => Some(project_s.clone()),
            "CLAUDE_CONFIG_DIR" => Some(config_s.clone()),
            "SLASH_COMMAND_TOOL_CHAR_BUDGET" => Some("12000".to_string()),
            _ => None,
        };
        let m = read_machine_settings(&env, tmp.path());
        assert_eq!(m.fraction, Some(0.02));
        assert_eq!(m.env_chars, Some(12_000));
        assert!(m.notes.iter().any(|n| n.contains("not read")));
    }

    #[test]
    fn machine_settings_ignores_bad_values() {
        let tmp = tempfile::tempdir().unwrap();
        let env = |k: &str| match k {
            "SLASH_COMMAND_TOOL_CHAR_BUDGET" => Some("lots".to_string()),
            "CLAUDE_PROJECT_DIR" => Some("/nonexistent".to_string()),
            "CLAUDE_CONFIG_DIR" => Some("/nonexistent".to_string()),
            _ => None,
        };
        let m = read_machine_settings(&env, tmp.path());
        assert_eq!(m.env_chars, None);
        assert_eq!(m.fraction, None);
    }
}
