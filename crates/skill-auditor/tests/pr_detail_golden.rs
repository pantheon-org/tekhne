//! Golden output for the pull request detail blocks.
//!
//! Scores two frozen corpus fixtures (one B, one B+) and compares the rendered
//! blocks with the files under `tests/golden-corpus/pr-detail/`. The fixtures do
//! not change, so a diff here means the scorer or the renderer changed. After
//! reviewing the change, regenerate with:
//!
//! ```text
//! BLESS_GOLDENS=1 cargo test -p pantheon-skill-auditor --test pr_detail_golden
//! ```

use std::fs;
use std::path::PathBuf;

use skill_auditor::reporter::pr_detail;
use skill_auditor::scorer::score_with_date;

/// Any fixed date works: the rendered blocks do not contain it.
const PINNED_DATE: &str = "2026-07-22";

const FIXTURES: &[&str] = &[
    "grade-b-google-scholar-search",
    "grade-bplus-moscow-prioritization",
];

fn corpus_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/golden-corpus")
}

/// Render the blocks for one fixture. The scored path is replaced with a stable
/// one, because the real one is an absolute path on whichever machine ran this.
fn render(fixture: &str) -> String {
    let skill_path = corpus_dir().join("fixtures").join(fixture).join("SKILL.md");
    let mut result = score_with_date(&skill_path, PINNED_DATE)
        .unwrap_or_else(|e| panic!("score {}: {e}", skill_path.display()));
    result.skill = format!("/runner/skills/golden/{fixture}/SKILL.md");
    pr_detail(&[result], usize::MAX)
}

#[test]
fn rendered_blocks_match_the_committed_goldens() {
    let golden_dir = corpus_dir().join("pr-detail");
    if std::env::var_os("BLESS_GOLDENS").is_some() {
        fs::create_dir_all(&golden_dir).expect("create golden dir");
        for fixture in FIXTURES {
            fs::write(golden_dir.join(format!("{fixture}.txt")), render(fixture))
                .expect("write golden");
        }
        eprintln!("BLESSED {} pr-detail goldens", FIXTURES.len());
        return;
    }
    for fixture in FIXTURES {
        let path = golden_dir.join(format!("{fixture}.txt"));
        let want =
            fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        assert_eq!(render(fixture), want, "{fixture} differs from its golden");
    }
}

#[test]
fn each_fixture_renders_exactly_one_block() {
    for fixture in FIXTURES {
        assert_eq!(render(fixture).matches("<details>").count(), 1, "{fixture}");
    }
}
