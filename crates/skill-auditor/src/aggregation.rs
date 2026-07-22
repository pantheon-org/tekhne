//! Aggregation planning for a skill "family" (skills sharing a name prefix).
//!
//! Ports `plan-aggregation.sh`. Two deliberate deviations from the shell:
//! - **Layout fix**: the shell scanned only the immediate children of each
//!   root (`skills/*`), so on this repo's nested `skills/<domain>/<skill>/`
//!   layout it matched nothing. This recurses and matches a skill by its
//!   directory name at any depth.
//! - **Engine reuse**: the average-duplication metric is computed with the
//!   shared [`crate::duplication`] line-overlap engine rather than the shell's
//!   bespoke ">10-char, lowercased, unique" normalisation, so there is one
//!   similarity implementation. The exact percentage therefore differs from the
//!   shell's, but it drives the same advisory thresholds.

use std::path::{Path, PathBuf};

use crate::duplication;

/// A skill belonging to the requested family.
#[derive(Debug, Clone)]
pub struct FamilySkill {
    /// Directory name (e.g. `bdd-testing`).
    pub name: String,
    /// Path to the skill's SKILL.md.
    pub path: PathBuf,
    /// Line count of SKILL.md (`wc -l`).
    pub lines: usize,
    /// The `description:` frontmatter value, if present.
    pub description: String,
    /// Raw SKILL.md content (used for the duplication metric).
    pub content: String,
}

/// The default roots searched for family members (matching the shell).
pub fn default_roots() -> Vec<PathBuf> {
    vec![PathBuf::from("skills"), PathBuf::from(".agents/skills")]
}

/// Recursively find skills under `roots` whose directory name is exactly
/// `prefix` or begins with `prefix-`. Results are sorted by name.
pub fn find_family(roots: &[PathBuf], prefix: &str) -> Vec<FamilySkill> {
    let mut out = Vec::new();
    for root in roots {
        collect_family(root, prefix, &mut out);
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

fn collect_family(dir: &Path, prefix: &str, out: &mut Vec<FamilySkill>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        let skill_md = path.join("SKILL.md");
        if skill_md.is_file() && name_in_family(&name, prefix) {
            if let Ok(content) = std::fs::read_to_string(&skill_md) {
                out.push(FamilySkill {
                    lines: content.lines().count(),
                    description: frontmatter_description(&content),
                    name,
                    path: skill_md,
                    content,
                });
            }
        } else {
            // Recurse into non-matching directories to reach nested skills.
            collect_family(&path, prefix, out);
        }
    }
}

/// A name is in the family when it equals the prefix or starts with `prefix-`.
fn name_in_family(name: &str, prefix: &str) -> bool {
    name == prefix || name.starts_with(&format!("{prefix}-"))
}

/// Extract the `description:` frontmatter value (first occurrence).
fn frontmatter_description(content: &str) -> String {
    content
        .lines()
        .find_map(|l| l.strip_prefix("description:"))
        .map(|v| v.trim().to_string())
        .unwrap_or_default()
}

/// Average pairwise line-overlap similarity (percent) across the family, via
/// the shared duplication engine. Returns 0 for fewer than two skills.
pub fn average_duplication(skills: &[FamilySkill]) -> u32 {
    if skills.len() < 2 {
        return 0;
    }
    let mut total = 0u32;
    let mut pairs = 0u32;
    for i in 0..skills.len() {
        for j in (i + 1)..skills.len() {
            let (sim, _) = duplication::basic_similarity(&skills[i].content, &skills[j].content);
            total += sim;
            pairs += 1;
        }
    }
    total.checked_div(pairs).unwrap_or(0)
}

/// Propose category names from the family members' name suffixes: for
/// `prefix-<cat>-...` the category is `<cat>`. Falls back to `core`/`advanced`
/// when no hyphenated members exist (ports the shell's default).
pub fn propose_categories(skills: &[FamilySkill]) -> Vec<String> {
    let mut cats: Vec<String> = Vec::new();
    for s in skills {
        // Strip everything up to and including the first hyphen.
        if let Some((_, rest)) = s.name.split_once('-') {
            let cat = rest.split('-').next().unwrap_or("");
            if !cat.is_empty() {
                cats.push(cat.to_string());
            }
        }
    }
    cats.sort();
    cats.dedup();
    if cats.is_empty() {
        vec!["core".to_string(), "advanced".to_string()]
    } else {
        cats
    }
}

/// Render the aggregation plan as Markdown (ports the shell's `generate_plan`).
pub fn render_plan(family: &str, skills: &[FamilySkill], duplication: u32) -> String {
    let total_lines: usize = skills.iter().map(|s| s.lines).sum();
    let skill_count = skills.len();
    let categories = propose_categories(skills);

    let hub_lines = 65usize;
    let refs = ((total_lines as f64 / 350.0).round() as usize).max(1);
    let new_total = hub_lines + refs * 200;
    let reduction = if total_lines > 0 {
        format!(
            "{:.1}",
            (1.0 - new_total as f64 / total_lines as f64) * 100.0
        )
    } else {
        "0.0".to_string()
    };

    let mut s = String::new();
    s.push_str(&format!("# Aggregation Plan: {family}\n\n## Summary\n\n"));
    s.push_str(&format!("- **Source Skills**: {skill_count}\n"));
    s.push_str(&format!("- **Total Lines**: {total_lines}\n"));
    s.push_str(&format!("- **Estimated Hub**: ~{hub_lines} lines\n"));
    s.push_str(&format!("- **Estimated References**: ~{refs} files\n"));
    s.push_str(&format!("- **Estimated Reduction**: {reduction}%\n\n"));

    s.push_str(
        "## Source Skills\n\n| Skill | Lines | Description |\n|-------|-------|-------------|\n",
    );
    for skill in skills {
        let desc = if skill.description.chars().count() > 50 {
            let short: String = skill.description.chars().take(50).collect();
            format!("{short}...")
        } else {
            skill.description.clone()
        };
        s.push_str(&format!(
            "| {} | {} | {} |\n",
            skill.name, skill.lines, desc
        ));
    }

    s.push_str("\n## Proposed Categories\n\n| Priority | Category | Prefix | Suggested Content |\n|----------|----------|--------|-------------------|\n");
    for (idx, cat) in categories.iter().enumerate() {
        let priority = match idx {
            0 => "CRITICAL",
            1 => "HIGH",
            2 => "MEDIUM",
            _ => "LOW",
        };
        s.push_str(&format!("| {priority} | {cat} | {cat}- | TBD |\n"));
    }

    s.push_str("\n## Implementation Steps\n\n");
    s.push_str("1. **Design Structure** - Finalize category organization\n");
    s.push_str("2. **Create Hub** - Write SKILL.md navigation (60-90 lines)\n");
    s.push_str("3. **Create AGENTS.md** - Reference guide with file listing\n");
    s.push_str("4. **Extract References** - Move content to categorized files\n");
    s.push_str("5. **Deprecate Sources** - Move to .deprecated/\n");
    s.push_str("6. **Verify** - Run quality evaluation\n\n");

    s.push_str("## Recommendations\n\n");
    let mut n = 1;
    if skill_count < 3 {
        s.push_str(&format!(
            "{n}. Only 1-2 skills found - consider if aggregation is necessary\n"
        ));
        n += 1;
    } else if skill_count >= 6 {
        s.push_str(&format!(
            "{n}. {skill_count} skills found - strong candidate for aggregation\n"
        ));
        n += 1;
    }
    if duplication > 30 {
        s.push_str(&format!(
            "{n}. High duplication ({duplication}%) - high priority for consolidation\n"
        ));
        n += 1;
    } else if duplication > 20 {
        s.push_str(&format!(
            "{n}. Moderate duplication ({duplication}%) - good candidate for consolidation\n"
        ));
        n += 1;
    } else {
        s.push_str(&format!(
            "{n}. Low duplication ({duplication}%) - review if skills are truly related\n"
        ));
        n += 1;
    }
    if total_lines > 2000 {
        s.push_str(&format!(
            "{n}. Large collection ({total_lines} lines) - significant reduction potential\n"
        ));
        n += 1;
    }
    s.push_str(&format!(
        "{n}. Follow aggregation-implementation.md 6-step process\n"
    ));
    n += 1;
    s.push_str(&format!(
        "{n}. Create navigation hub (SKILL.md) with priority categories\n"
    ));
    n += 1;
    s.push_str(&format!(
        "{n}. Extract content to references/ directory by category\n"
    ));

    s.push_str("\n## Next Actions\n\n");
    s.push_str("- [ ] Review this plan with team\n");
    s.push_str("- [ ] Refine category structure\n");
    s.push_str("- [ ] Estimate effort (hours)\n");
    s.push_str("- [ ] Schedule implementation\n");
    s.push_str("- [ ] Execute aggregation-implementation.md\n\n");
    s.push_str("---\n\n*Generated by skill-quality-auditor*\n");

    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn mk_skill(root: &Path, rel: &str, body: &str) {
        let dir = root.join(rel);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("SKILL.md"), body).unwrap();
    }

    #[test]
    fn name_in_family_matches_prefix_and_hyphen() {
        assert!(name_in_family("bdd", "bdd"));
        assert!(name_in_family("bdd-testing", "bdd"));
        assert!(!name_in_family("bddx", "bdd"));
        assert!(!name_in_family("other", "bdd"));
    }

    #[test]
    fn find_family_recurses_nested_layout() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();
        // Nested domain/skill layout that the shell could not handle.
        mk_skill(
            root,
            "skills/testing/bdd-testing",
            "---\ndescription: bdd core\n---\nbody\n",
        );
        mk_skill(
            root,
            "skills/testing/bdd-unit",
            "---\ndescription: bdd units\n---\nbody\n",
        );
        mk_skill(
            root,
            "skills/other/unrelated",
            "---\ndescription: no\n---\nbody\n",
        );

        let found = find_family(&[root.join("skills")], "bdd");
        let names: Vec<&str> = found.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(names, vec!["bdd-testing", "bdd-unit"]);
        assert_eq!(found[0].description, "bdd core");
    }

    #[test]
    fn propose_categories_from_suffixes() {
        let skills = vec![
            FamilySkill {
                name: "bdd-testing".into(),
                path: "x".into(),
                lines: 1,
                description: String::new(),
                content: String::new(),
            },
            FamilySkill {
                name: "bdd-unit-runner".into(),
                path: "x".into(),
                lines: 1,
                description: String::new(),
                content: String::new(),
            },
        ];
        assert_eq!(propose_categories(&skills), vec!["testing", "unit"]);
    }

    #[test]
    fn propose_categories_default_when_no_suffix() {
        let skills = vec![FamilySkill {
            name: "bdd".into(),
            path: "x".into(),
            lines: 1,
            description: String::new(),
            content: String::new(),
        }];
        assert_eq!(propose_categories(&skills), vec!["core", "advanced"]);
    }

    #[test]
    fn average_duplication_zero_for_single_skill() {
        let skills = vec![FamilySkill {
            name: "bdd".into(),
            path: "x".into(),
            lines: 1,
            description: String::new(),
            content: "a\nb\n".into(),
        }];
        assert_eq!(average_duplication(&skills), 0);
    }

    #[test]
    fn render_plan_contains_summary_and_reduction() {
        let skills = vec![
            FamilySkill {
                name: "bdd-testing".into(),
                path: "x".into(),
                lines: 700,
                description: "short".into(),
                content: "one\ntwo\n".into(),
            },
            FamilySkill {
                name: "bdd-unit".into(),
                path: "x".into(),
                lines: 700,
                description: "short".into(),
                content: "one\ntwo\n".into(),
            },
        ];
        let out = render_plan("bdd", &skills, average_duplication(&skills));
        assert!(out.contains("# Aggregation Plan: bdd"));
        assert!(out.contains("- **Source Skills**: 2"));
        assert!(out.contains("- **Total Lines**: 1400"));
        // refs = round(1400/350)=4, new_total=65+800=865, reduction=(1-865/1400)*100=38.2
        assert!(out.contains("Estimated Reduction**: 38.2%"), "got:\n{out}");
    }
}
