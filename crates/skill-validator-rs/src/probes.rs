//! Invocation probes: would a skill's listing text win the requests it should,
//! and stay quiet on the ones it should not?
//!
//! Static gates (length caps, trigger-phrase drift, the shared budget) cannot
//! say whether a description routes well. This module scores labelled requests
//! against a skill's listing text and its named competitors with a
//! deterministic **lexical floor**. The floor is not a model-graded
//! auto-invocation rate: a high rate means the listing already contains the
//! request's nouns, and a low one means a rewrite still has room. It never
//! calls a model and never decides a gate.
//!
//! Ported from the `skill-quality` plugin's `measure-invocation.sh`, with one
//! deliberate difference: each distinct listing token counts once, so repeating
//! a word in a description does not raise its score.
//!
//! Probe files live in one directory, one JSON file per skill (not
//! `baselines/`). Paths in a probe file are relative to the repository root.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::skill::Skill;
use crate::triggers;

/// The scoring method's name, recorded in every report.
pub const METHOD: &str = "listing-overlap";
/// Fewer queries than this is a validation error.
pub const MIN_QUERIES: usize = 8;
/// The recommended query count range; outside it is a warning.
pub const RECOMMENDED_QUERIES: (usize, usize) = (16, 24);
/// A should-trigger probe sharing this many consecutive words with the listing
/// is flagged as a copy.
pub const DEFAULT_COPY_SPAN: usize = 4;
/// Each matched trigger phrase is worth this many matched tokens.
const PHRASE_WEIGHT: usize = 10;
/// Tokens shorter than this carry no signal.
const MIN_TOKEN_CHARS: usize = 3;

const STOPWORDS: &[&str] = &[
    "the", "and", "for", "with", "when", "use", "this", "that", "from", "into", "not", "your",
    "our", "are", "was", "were", "has", "have", "had", "its", "but", "nor", "any", "all", "can",
    "may", "per", "via", "than", "then", "also", "just", "only", "each", "both", "same", "such",
    "over", "under", "after", "before", "about",
];

/// Which half of the probe set a query belongs to. A rewrite is tuned against
/// `train` and judged on `validation`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Split {
    /// Queries a rewrite may be tuned against.
    Train,
    /// Queries held out to judge a rewrite.
    Validation,
}

/// One labelled request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Query {
    /// Unique within the file.
    pub id: String,
    /// Train or validation.
    pub split: Split,
    /// Whether the target skill should win this request.
    pub expect_trigger: bool,
    /// The request, worded the way a user would ask.
    pub request: String,
}

/// One probe file: a target skill, its rivals and its labelled queries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProbeFile {
    /// The report name for the target skill.
    pub skill: String,
    /// Repository-relative directory holding the target's `SKILL.md`.
    pub skill_dir: String,
    /// Repository-relative directories of competing skills.
    #[serde(default)]
    pub competitors: Vec<String>,
    /// Why this skill was chosen (free text, not scored).
    #[serde(default)]
    pub note: Option<String>,
    /// The labelled requests.
    pub queries: Vec<Query>,
}

/// A validation finding. Errors fail `probes validate`; warnings do not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// True for an error.
    pub error: bool,
    /// The probe file the finding is about.
    pub file: String,
    /// What is wrong.
    pub message: String,
}

/// Load every `*.json` file directly in `dir` (hidden files and subdirectories,
/// such as `baselines/`, are skipped), sorted by file name.
pub fn load_dir(dir: &Path) -> Result<Vec<(String, ProbeFile)>, String> {
    let mut names: Vec<PathBuf> = std::fs::read_dir(dir)
        .map_err(|e| format!("reading {}: {e}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.is_file()
                && p.extension().is_some_and(|x| x == "json")
                && !p
                    .file_name()
                    .is_some_and(|n| n.to_string_lossy().starts_with('.'))
        })
        .collect();
    names.sort();
    let mut out = Vec::new();
    for path in names {
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let text = std::fs::read_to_string(&path).map_err(|e| format!("reading {name}: {e}"))?;
        let probe: ProbeFile =
            serde_json::from_str(&text).map_err(|e| format!("parsing {name}: {e}"))?;
        out.push((name, probe));
    }
    if out.is_empty() {
        return Err(format!("no probe files (*.json) in {}", dir.display()));
    }
    Ok(out)
}

/// The listing text for the skill in `repo_root/skill_dir`.
fn listing_for(repo_root: &Path, skill_dir: &str) -> Result<String, String> {
    let dir = repo_root.join(skill_dir);
    let skill = Skill::load(&dir).map_err(|e| format!("{skill_dir}: {e}"))?;
    Ok(triggers::listing_text(&skill))
}

/// Lowercased alphanumeric words of `text`, in order.
fn words(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(str::to_string)
        .collect()
}

/// The distinct content tokens of a listing: words of at least
/// [`MIN_TOKEN_CHARS`] characters that are not stopwords.
fn listing_tokens(listing: &str) -> BTreeSet<String> {
    words(listing)
        .into_iter()
        .filter(|w| w.len() >= MIN_TOKEN_CHARS && !STOPWORDS.contains(&w.as_str()))
        .collect()
}

/// The lexical score of `request` against `listing`: matched trigger phrases
/// (weighted) plus matched content tokens, by substring of the lowercased
/// request.
pub fn score_request(request: &str, listing: &str) -> usize {
    let req = request.to_lowercase();
    let phrase_hits = triggers::phrases(listing)
        .iter()
        .filter(|p| req.contains(p.as_str()))
        .count();
    let token_hits = listing_tokens(listing)
        .iter()
        .filter(|t| req.contains(t.as_str()))
        .count();
    phrase_hits * PHRASE_WEIGHT + token_hits
}

/// Validate every probe file. `copy_span` is the consecutive-word count that
/// flags a should-trigger probe as copied from the listing.
pub fn validate(files: &[(String, ProbeFile)], repo_root: &Path, copy_span: usize) -> Vec<Finding> {
    let mut out = Vec::new();
    for (file, p) in files {
        let mut err = |m: String| {
            out.push(Finding {
                error: true,
                file: file.clone(),
                message: m,
            })
        };
        let mut warns: Vec<String> = Vec::new();

        if p.skill.trim().is_empty() {
            err("skill is empty".to_string());
        }
        let n = p.queries.len();
        if n < MIN_QUERIES {
            err(format!("{n} queries, need at least {MIN_QUERIES}"));
        } else if n < RECOMMENDED_QUERIES.0 || n > RECOMMENDED_QUERIES.1 {
            warns.push(format!(
                "{n} queries; {}-{} is the recommended range",
                RECOMMENDED_QUERIES.0, RECOMMENDED_QUERIES.1
            ));
        }

        let mut seen = BTreeSet::new();
        for q in &p.queries {
            if !seen.insert(q.id.as_str()) {
                err(format!("duplicate query id {}", q.id));
            }
            if q.request.trim().is_empty() {
                err(format!("query {} has an empty request", q.id));
            }
        }

        for (label, want) in [("should-trigger", true), ("should-not-trigger", false)] {
            if !p.queries.iter().any(|q| q.expect_trigger == want) {
                err(format!("no {label} queries"));
            }
        }
        for split in [Split::Train, Split::Validation] {
            for (label, want) in [("should-trigger", true), ("should-not-trigger", false)] {
                if !p
                    .queries
                    .iter()
                    .any(|q| q.split == split && q.expect_trigger == want)
                {
                    warns.push(format!("{split:?} split has no {label} queries"));
                }
            }
        }

        match listing_for(repo_root, &p.skill_dir) {
            Err(e) => err(format!("target skill: {e}")),
            Ok(listing) => {
                let listing_words = words(&listing);
                for q in p.queries.iter().filter(|q| q.expect_trigger) {
                    if let Some(span) = shared_span(&words(&q.request), &listing_words, copy_span) {
                        warns.push(format!(
                            "should-trigger probe {} copies {copy_span} consecutive listing \
                             words (\"{span}\"); word it the way a user would",
                            q.id
                        ));
                    }
                }
            }
        }
        for c in &p.competitors {
            if let Err(e) = listing_for(repo_root, c) {
                err(format!("competitor: {e}"));
            }
        }

        for w in warns {
            out.push(Finding {
                error: false,
                file: file.clone(),
                message: w,
            });
        }
    }
    out
}

/// The first run of `span` consecutive request words that also appears
/// consecutively in the listing words.
fn shared_span(req: &[String], listing: &[String], span: usize) -> Option<String> {
    if span == 0 || req.len() < span || listing.len() < span {
        return None;
    }
    let grams: BTreeSet<&[String]> = listing.windows(span).collect();
    req.windows(span)
        .find(|w| grams.contains(w))
        .map(|w| w.join(" "))
}

/// Per-split counts and rates.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SplitStats {
    /// Queries in the split.
    pub n: usize,
    /// Should-trigger queries.
    pub n_positive: usize,
    /// Should-not-trigger queries.
    pub n_negative: usize,
    /// Should-trigger queries the target won.
    pub trigger_hits: usize,
    /// Should-not-trigger queries the target won.
    pub false_triggers: usize,
    /// `trigger_hits / n_positive`, null with no positives.
    pub trigger_rate: Option<f64>,
    /// `false_triggers / n_negative`, null with no negatives.
    pub false_trigger_rate: Option<f64>,
}

/// One scored query.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Case {
    /// Query id.
    pub id: String,
    /// Train or validation.
    pub split: Split,
    /// The label.
    pub expect_trigger: bool,
    /// Whether the target uniquely won.
    pub predicted: bool,
    /// Whether the prediction matched the label.
    pub correct: bool,
    /// The target's score.
    pub target_score: usize,
    /// The winning skill, `none`, or `target+competitor` on a tie.
    pub winner: String,
    /// The request text.
    pub request: String,
}

/// One skill's scored results.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillScore {
    /// The target's report name.
    pub skill: String,
    /// Stats per split.
    pub splits: BTreeMap<Split, SplitStats>,
    /// Every scored query.
    pub cases: Vec<Case>,
}

/// A scoring run over a probe directory.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScoreReport {
    /// The method name ([`METHOD`]).
    pub method: String,
    /// Always states that the floor is lexical, so no reader mistakes it for a
    /// model-graded rate.
    pub note: String,
    /// Per-skill results.
    pub skills: Vec<SkillScore>,
}

/// Decide who wins one request: the target wins only with a positive score
/// that no competitor matches or beats. Competitors are tried in name order, as
/// the original does.
fn decide(
    skill: &str,
    request: &str,
    listing: &str,
    competitors: &BTreeMap<String, String>,
) -> (usize, bool, String) {
    let target = score_request(request, listing);
    let (mut winner, mut best, mut predicted) = if target == 0 {
        ("none".to_string(), 0, false)
    } else {
        (skill.to_string(), target, true)
    };
    for (name, comp_listing) in competitors {
        let cs = score_request(request, comp_listing);
        if cs > best {
            best = cs;
            winner = name.clone();
            predicted = false;
        } else if cs == best && winner == skill && cs > 0 {
            predicted = false;
            winner = format!("{skill}+{name}");
        }
    }
    (target, predicted, winner)
}

fn rate(hits: usize, n: usize) -> Option<f64> {
    (n > 0).then(|| hits as f64 / n as f64)
}

/// Score every probe file.
pub fn score(files: &[(String, ProbeFile)], repo_root: &Path) -> Result<ScoreReport, String> {
    let mut skills = Vec::new();
    for (file, p) in files {
        let listing = listing_for(repo_root, &p.skill_dir).map_err(|e| format!("{file}: {e}"))?;
        let mut competitors = BTreeMap::new();
        for c in &p.competitors {
            let l = listing_for(repo_root, c).map_err(|e| format!("{file}: competitor {e}"))?;
            competitors.insert(c.clone(), l);
        }

        let mut cases = Vec::new();
        for q in &p.queries {
            let (target_score, predicted, winner) =
                decide(&p.skill, &q.request, &listing, &competitors);
            cases.push(Case {
                id: q.id.clone(),
                split: q.split,
                expect_trigger: q.expect_trigger,
                predicted,
                correct: predicted == q.expect_trigger,
                target_score,
                winner,
                request: q.request.clone(),
            });
        }

        let mut splits = BTreeMap::new();
        for split in [Split::Train, Split::Validation] {
            let in_split: Vec<&Case> = cases.iter().filter(|c| c.split == split).collect();
            let pos: Vec<&&Case> = in_split.iter().filter(|c| c.expect_trigger).collect();
            let neg: Vec<&&Case> = in_split.iter().filter(|c| !c.expect_trigger).collect();
            let hits = pos.iter().filter(|c| c.predicted).count();
            let fals = neg.iter().filter(|c| c.predicted).count();
            splits.insert(
                split,
                SplitStats {
                    n: in_split.len(),
                    n_positive: pos.len(),
                    n_negative: neg.len(),
                    trigger_hits: hits,
                    false_triggers: fals,
                    trigger_rate: rate(hits, pos.len()),
                    false_trigger_rate: rate(fals, neg.len()),
                },
            );
        }
        skills.push(SkillScore {
            skill: p.skill.clone(),
            splits,
            cases,
        });
    }
    Ok(ScoreReport {
        method: METHOD.to_string(),
        note: "Lexical floor, not a model-graded invocation rate: a high trigger rate means \
               the listing already contains the request's words."
            .to_string(),
        skills,
    })
}

/// A paired 95% normal-approximation interval on a trigger-rate delta.
pub type Interval = [f64; 2];

/// Treatment minus baseline for one split.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SplitDelta {
    /// Baseline trigger rate.
    pub trigger_rate_baseline: Option<f64>,
    /// Treatment trigger rate.
    pub trigger_rate_treatment: Option<f64>,
    /// Treatment minus baseline.
    pub trigger_rate_delta: Option<f64>,
    /// Paired 95% interval over the per-probe hit changes, clamped to [-1, 1];
    /// null with fewer than 2 paired probes or when the positive ids differ.
    pub trigger_rate_delta_interval: Option<Interval>,
    /// True when the interval contains 0.
    pub trigger_rate_within_noise: Option<bool>,
    /// Baseline false-trigger rate.
    pub false_trigger_rate_baseline: Option<f64>,
    /// Treatment false-trigger rate.
    pub false_trigger_rate_treatment: Option<f64>,
    /// Treatment minus baseline.
    pub false_trigger_rate_delta: Option<f64>,
}

/// One skill's deltas.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SkillDelta {
    /// The skill's report name.
    pub skill: String,
    /// Train deltas.
    pub train: SplitDelta,
    /// Validation deltas.
    pub validation: SplitDelta,
}

/// The comparison of two score reports.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CompareReport {
    /// Per-skill deltas.
    pub skills: Vec<SkillDelta>,
}

fn r3(x: f64) -> f64 {
    (x * 1000.0).round() / 1000.0
}

fn diff(t: Option<f64>, b: Option<f64>) -> Option<f64> {
    Some(t? - b?)
}

/// The paired interval over positive probes of `split`, matched by id.
fn interval(base: &SkillScore, treat: &SkillScore, split: Split) -> Option<Interval> {
    let pos = |s: &SkillScore| -> BTreeMap<String, f64> {
        s.cases
            .iter()
            .filter(|c| c.split == split && c.expect_trigger)
            .map(|c| (c.id.clone(), if c.predicted { 1.0 } else { 0.0 }))
            .collect()
    };
    let positives = |s: &SkillScore| {
        s.cases
            .iter()
            .filter(|c| c.split == split && c.expect_trigger)
            .count()
    };
    let (b, t) = (pos(base), pos(treat));
    // Duplicate ids collapse in the map, so the counts catch them.
    if b.len() != positives(base) || t.len() != positives(treat) {
        return None;
    }
    if b.len() < 2 || b.keys().ne(t.keys()) {
        return None;
    }
    let d: Vec<f64> = t.iter().map(|(id, hit)| hit - b[id]).collect();
    let n = d.len() as f64;
    let mean = d.iter().sum::<f64>() / n;
    let var = d.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0);
    let half = var.sqrt() * 1.96 / n.sqrt();
    Some([r3((mean - half).max(-1.0)), r3((mean + half).min(1.0))])
}

fn split_delta(base: &SkillScore, treat: &SkillScore, split: Split) -> SplitDelta {
    let empty = SplitStats::default();
    let b = base.splits.get(&split).unwrap_or(&empty);
    let t = treat.splits.get(&split).unwrap_or(&empty);
    let iv = interval(base, treat, split);
    SplitDelta {
        trigger_rate_baseline: b.trigger_rate,
        trigger_rate_treatment: t.trigger_rate,
        trigger_rate_delta: diff(t.trigger_rate, b.trigger_rate),
        trigger_rate_within_noise: iv.map(|i| i[0] <= 0.0 && i[1] >= 0.0),
        trigger_rate_delta_interval: iv,
        false_trigger_rate_baseline: b.false_trigger_rate,
        false_trigger_rate_treatment: t.false_trigger_rate,
        false_trigger_rate_delta: diff(t.false_trigger_rate, b.false_trigger_rate),
    }
}

/// Compare two score reports over the same non-empty skill set.
pub fn compare(base: &ScoreReport, treat: &ScoreReport) -> Result<CompareReport, String> {
    let names = |r: &ScoreReport| -> BTreeSet<String> {
        r.skills.iter().map(|s| s.skill.clone()).collect()
    };
    if base.skills.is_empty() || names(base) != names(treat) {
        return Err("compare needs two reports over the same non-empty skill set".to_string());
    }
    let skills = treat
        .skills
        .iter()
        .map(|t| {
            let b = base.skills.iter().find(|b| b.skill == t.skill).unwrap();
            SkillDelta {
                skill: t.skill.clone(),
                train: split_delta(b, t, Split::Train),
                validation: split_delta(b, t, Split::Validation),
            }
        })
        .collect();
    Ok(CompareReport { skills })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn skill(root: &Path, rel: &str, description: &str) {
        let dir = root.join(rel);
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("SKILL.md"),
            format!("---\nname: s\ndescription: |-\n  {description}\n---\n\nBody\n"),
        )
        .unwrap();
    }

    fn q(id: &str, split: Split, expect: bool, request: &str) -> Query {
        Query {
            id: id.into(),
            split,
            expect_trigger: expect,
            request: request.into(),
        }
    }

    fn probe(queries: Vec<Query>) -> ProbeFile {
        ProbeFile {
            skill: "t:gen".into(),
            skill_dir: "gen".into(),
            competitors: vec!["val".into()],
            note: None,
            queries,
        }
    }

    fn fixture() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        skill(
            tmp.path(),
            "gen",
            "Generate terraform modules. Use when writing terraform resources",
        );
        skill(
            tmp.path(),
            "val",
            "Validate terraform configs. Use when linting terraform plans",
        );
        tmp
    }

    #[test]
    fn score_counts_tokens_and_phrases() {
        let l = "Generate terraform modules. Use when writing terraform resources";
        assert!(score_request("generate a terraform module", l) >= 2);
        assert_eq!(score_request("bake a cake", l), 0);
        // A matched trigger phrase is weighted well above one token.
        assert!(score_request("help with writing terraform resources", l) >= PHRASE_WEIGHT);
    }

    #[test]
    fn repeating_a_word_in_the_listing_does_not_raise_the_score() {
        let once = score_request("terraform", "terraform tool");
        let many = score_request("terraform", "terraform terraform terraform tool");
        assert_eq!(once, many);
    }

    #[test]
    fn target_wins_only_when_no_competitor_matches_or_beats_it() {
        let mut comps = BTreeMap::new();
        comps.insert("val".to_string(), "linting terraform plans".to_string());
        let (_, won, w) = decide(
            "gen",
            "generate terraform modules",
            "generate terraform modules",
            &comps,
        );
        assert!(won && w == "gen");
        let (_, won, w) = decide("gen", "lint terraform plans", "generate terraform", &comps);
        assert!(!won && w == "val");
        let (_, won, w) = decide("gen", "terraform", "terraform generator", &comps);
        assert!(!won, "tie must not count as a win, got {w}");
        let (s, won, w) = decide("gen", "bake a cake", "terraform", &comps);
        assert!(!won && w == "none" && s == 0);
    }

    #[test]
    fn score_reports_rates_per_split() {
        let tmp = fixture();
        let p = probe(vec![
            q(
                "p1",
                Split::Train,
                true,
                "generate terraform modules for me",
            ),
            q(
                "p2",
                Split::Validation,
                true,
                "make some resources in terraform",
            ),
            q("n1", Split::Train, false, "lint terraform plans"),
            q("n2", Split::Validation, false, "bake a cake"),
        ]);
        let r = score(&[("a.json".into(), p)], tmp.path()).unwrap();
        let s = &r.skills[0];
        assert_eq!(s.splits[&Split::Train].n, 2);
        assert_eq!(s.splits[&Split::Train].trigger_rate, Some(1.0));
        assert_eq!(s.splits[&Split::Train].false_trigger_rate, Some(0.0));
        assert_eq!(s.splits[&Split::Validation].false_trigger_rate, Some(0.0));
        assert_eq!(r.method, METHOD);
        assert!(r.note.contains("not a model-graded"));
    }

    #[test]
    fn validate_flags_count_labels_duplicates_and_missing_skills() {
        let tmp = fixture();
        let few = probe(vec![q("a", Split::Train, true, "x terraform")]);
        let f = validate(&[("few.json".into(), few)], tmp.path(), 4);
        assert!(f
            .iter()
            .any(|x| x.error && x.message.contains("need at least")));
        assert!(f
            .iter()
            .any(|x| x.error && x.message.contains("no should-not-trigger")));

        let mut dup: Vec<Query> = (0..9)
            .map(|i| {
                q(
                    &format!("p{i}"),
                    Split::Train,
                    i % 2 == 0,
                    &format!("req {i} terraform"),
                )
            })
            .collect();
        dup.push(q("p0", Split::Validation, false, "again"));
        let f = validate(&[("d.json".into(), probe(dup))], tmp.path(), 4);
        assert!(f
            .iter()
            .any(|x| x.error && x.message.contains("duplicate query id p0")));

        let mut missing = probe(
            (0..10)
                .map(|i| q(&format!("m{i}"), Split::Train, i % 2 == 0, "r terraform"))
                .collect(),
        );
        missing.skill_dir = "nope".into();
        let f = validate(&[("m.json".into(), missing)], tmp.path(), 4);
        assert!(f
            .iter()
            .any(|x| x.error && x.message.contains("target skill")));
    }

    #[test]
    fn validate_warns_when_a_positive_copies_the_listing() {
        let tmp = fixture();
        let mut qs: Vec<Query> = (0..9)
            .map(|i| {
                q(
                    &format!("q{i}"),
                    if i < 5 {
                        Split::Train
                    } else {
                        Split::Validation
                    },
                    i % 2 == 0,
                    &format!("something {i} else entirely"),
                )
            })
            .collect();
        qs[0] = q(
            "q0",
            Split::Train,
            true,
            "please generate terraform modules now",
        );
        let f = validate(&[("c.json".into(), probe(qs))], tmp.path(), 3);
        assert!(f
            .iter()
            .any(|x| !x.error && x.message.contains("q0") && x.message.contains("copies")));
    }

    fn report(rows: &[(&str, bool)]) -> ScoreReport {
        let cases: Vec<Case> = rows
            .iter()
            .map(|(id, hit)| Case {
                id: id.to_string(),
                split: Split::Train,
                expect_trigger: true,
                predicted: *hit,
                correct: *hit,
                target_score: 1,
                winner: "s".into(),
                request: "r".into(),
            })
            .collect();
        let hits = rows.iter().filter(|(_, h)| *h).count();
        let mut splits = BTreeMap::new();
        splits.insert(
            Split::Train,
            SplitStats {
                n: rows.len(),
                n_positive: rows.len(),
                trigger_hits: hits,
                trigger_rate: rate(hits, rows.len()),
                ..SplitStats::default()
            },
        );
        ScoreReport {
            method: METHOD.into(),
            note: String::new(),
            skills: vec![SkillScore {
                skill: "s".into(),
                splits,
                cases,
            }],
        }
    }

    #[test]
    fn compare_gives_delta_and_a_paired_interval() {
        let base = report(&[("a", false), ("b", false), ("c", true), ("d", false)]);
        let treat = report(&[("a", true), ("b", true), ("c", true), ("d", true)]);
        let c = compare(&base, &treat).unwrap();
        let d = &c.skills[0].train;
        assert_eq!(d.trigger_rate_delta, Some(0.75));
        let iv = d.trigger_rate_delta_interval.unwrap();
        assert!(iv[0] > 0.0 && iv[1] <= 1.0, "{iv:?}");
        assert_eq!(d.trigger_rate_within_noise, Some(false));
    }

    #[test]
    fn compare_calls_a_mixed_small_shift_within_noise() {
        let base = report(&[("a", true), ("b", false), ("c", true), ("d", false)]);
        let treat = report(&[("a", false), ("b", true), ("c", true), ("d", false)]);
        let d = compare(&base, &treat).unwrap().skills[0].train.clone();
        assert_eq!(d.trigger_rate_delta, Some(0.0));
        assert_eq!(d.trigger_rate_within_noise, Some(true));
    }

    #[test]
    fn compare_has_no_interval_with_one_probe_or_mismatched_ids() {
        let one = compare(&report(&[("a", false)]), &report(&[("a", true)])).unwrap();
        assert_eq!(one.skills[0].train.trigger_rate_delta_interval, None);
        let mismatch = compare(
            &report(&[("a", true), ("b", true)]),
            &report(&[("a", true), ("z", true)]),
        )
        .unwrap();
        assert_eq!(mismatch.skills[0].train.trigger_rate_delta_interval, None);
    }

    #[test]
    fn compare_rejects_different_skill_sets() {
        let a = report(&[("a", true)]);
        let mut b = report(&[("a", true)]);
        b.skills[0].skill = "other".into();
        assert!(compare(&a, &b).is_err());
        let empty = ScoreReport {
            method: METHOD.into(),
            note: String::new(),
            skills: vec![],
        };
        assert!(compare(&empty, &empty).is_err());
    }

    #[test]
    fn load_dir_skips_baselines_and_hidden_files() {
        let tmp = tempfile::tempdir().unwrap();
        let p = probe(vec![q("a", Split::Train, true, "x")]);
        fs::write(
            tmp.path().join("one.json"),
            serde_json::to_string(&p).unwrap(),
        )
        .unwrap();
        fs::write(tmp.path().join(".hidden.json"), "not json").unwrap();
        fs::create_dir_all(tmp.path().join("baselines")).unwrap();
        fs::write(tmp.path().join("baselines/b.json"), "not json").unwrap();
        let loaded = load_dir(tmp.path()).unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].0, "one.json");
        assert!(load_dir(&tmp.path().join("baselines")).is_err());
    }
}
