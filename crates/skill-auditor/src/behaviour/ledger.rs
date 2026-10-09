//! The behaviour report: for each scenario, every assertion graded against the
//! old trace and the new one, with a marker where something that passed before
//! fails now. It sits beside the score and is never part of it.

use std::path::Path;

use serde::Serialize;
use serde_json::Value;

use super::assertions::{evaluate, Kind};
use super::load::{load, Loaded};
use super::trace::{Source, Trace};
use super::SCHEMA_VERSION;

/// How one assertion fared across the two runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Verdict {
    /// Passed in both runs.
    Held,
    /// Passed in the old run and fails in the new one.
    Regression,
    /// Failed in the old run and passes in the new one.
    Fixed,
    /// Failed in both runs.
    Failing,
}

impl Verdict {
    fn of(old_pass: bool, new_pass: bool) -> Verdict {
        match (old_pass, new_pass) {
            (true, true) => Verdict::Held,
            (true, false) => Verdict::Regression,
            (false, true) => Verdict::Fixed,
            (false, false) => Verdict::Failing,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Verdict::Held => "held",
            Verdict::Regression => "REGRESSION",
            Verdict::Fixed => "fixed",
            Verdict::Failing => "failing",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Row {
    pub kind: Kind,
    pub expected: String,
    pub old_pass: bool,
    pub new_pass: bool,
    pub verdict: Verdict,
}

/// Where a trace came from, copied from its header.
#[derive(Debug, Clone, Serialize)]
pub struct TraceNote {
    pub harness: String,
    pub date: String,
    pub skill_commit: String,
    pub source: Source,
}

impl TraceNote {
    fn of(trace: &Trace) -> TraceNote {
        TraceNote {
            harness: trace.header.harness.clone(),
            date: trace.header.date.clone(),
            skill_commit: trace.header.skill_commit.clone(),
            source: trace.header.source,
        }
    }

    fn describe(&self) -> String {
        format!(
            "{}, {}, skill {}, {}",
            self.harness,
            self.date,
            self.skill_commit,
            self.source.label()
        )
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ScenarioLedger {
    /// The scenario folder, for example `scenario-1`.
    pub scenario: String,
    /// The name given in `behaviour.yaml`.
    pub name: String,
    pub old: TraceNote,
    pub new: TraceNote,
    pub rows: Vec<Row>,
    pub regressions: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct Report {
    pub schema_version: u32,
    pub scenarios: Vec<ScenarioLedger>,
}

/// Grade one scenario's expectations against its old and new traces.
pub fn compare(scenario: &str, loaded: &Loaded) -> ScenarioLedger {
    let old = evaluate(&loaded.old, &loaded.behaviour.expect);
    let new = evaluate(&loaded.new, &loaded.behaviour.expect);
    let rows: Vec<Row> = old
        .into_iter()
        .zip(new)
        .map(|(o, n)| Row {
            kind: o.kind,
            expected: o.expected,
            old_pass: o.pass,
            new_pass: n.pass,
            verdict: Verdict::of(o.pass, n.pass),
        })
        .collect();
    let regressions = rows
        .iter()
        .filter(|r| r.verdict == Verdict::Regression)
        .count();
    ScenarioLedger {
        scenario: scenario.to_string(),
        name: loaded.behaviour.name.clone(),
        old: TraceNote::of(&loaded.old),
        new: TraceNote::of(&loaded.new),
        rows,
        regressions,
    }
}

/// The report for a skill folder, or `None` when none of its scenarios carries
/// valid behaviour files. A broken scenario is left out and is reported as a D9
/// warning elsewhere; nothing is guessed from it.
pub fn report_for_skill(skill_dir: &Path) -> Option<Report> {
    let read = std::fs::read_dir(skill_dir.join("evals")).ok()?;
    let mut dirs: Vec<_> = read
        .flatten()
        .filter(|e| e.path().is_dir() && e.file_name().to_string_lossy().starts_with("scenario-"))
        .collect();
    dirs.sort_by_key(|e| e.file_name());

    let scenarios: Vec<ScenarioLedger> = dirs
        .iter()
        .filter_map(|e| {
            let loaded = load(&e.path()).ok().flatten()?;
            Some(compare(&e.file_name().to_string_lossy(), &loaded))
        })
        .collect();
    if scenarios.is_empty() {
        None
    } else {
        Some(Report {
            schema_version: SCHEMA_VERSION,
            scenarios,
        })
    }
}

fn pass_word(pass: bool) -> &'static str {
    if pass {
        "pass"
    } else {
        "FAIL"
    }
}

/// The report as a table for the terminal.
pub fn format_text(report: &Report) -> String {
    let mut out = String::from("Behaviour checks (recorded traces, not part of the score)\n");
    for s in &report.scenarios {
        out.push_str(&format!("\n{}: {}\n", s.scenario, s.name));
        out.push_str(&format!("  old: {}\n", s.old.describe()));
        out.push_str(&format!("  new: {}\n", s.new.describe()));
        let labels: Vec<String> = s
            .rows
            .iter()
            .map(|r| format!("{}: {}", r.kind.label(), r.expected))
            .collect();
        let width = labels.iter().map(String::len).max().unwrap_or(0).max(40);
        out.push_str(&format!(
            "  {:<width$}  {:<4}  {:<4}  verdict\n",
            "assertion", "old", "new"
        ));
        for (label, r) in labels.iter().zip(&s.rows) {
            out.push_str(&format!(
                "  {:<width$}  {:<4}  {:<4}  {}\n",
                label,
                pass_word(r.old_pass),
                pass_word(r.new_pass),
                r.verdict.label()
            ));
        }
        let fixed = s
            .rows
            .iter()
            .filter(|r| r.verdict == Verdict::Fixed)
            .count();
        out.push_str(&format!(
            "  {} regression{}, {} fixed\n",
            s.regressions,
            if s.regressions == 1 { "" } else { "s" },
            fixed
        ));
    }
    out
}

/// Serialise `value` and, when there is a report, add it as a `behaviour` field.
/// Without a report the output is byte for byte what `value` serialises to, key
/// order included: only a skill that has behaviour files goes through a JSON
/// value, which sorts its keys.
pub fn to_json<T: Serialize>(value: &T, report: Option<&Report>) -> Result<String, String> {
    let Some(report) = report else {
        return serde_json::to_string_pretty(value).map_err(|e| format!("marshal result: {e}"));
    };
    let mut v: Value = serde_json::to_value(value).map_err(|e| format!("marshal result: {e}"))?;
    if let Value::Object(map) = &mut v {
        let r = serde_json::to_value(report).map_err(|e| format!("marshal behaviour: {e}"))?;
        map.insert("behaviour".to_string(), r);
    }
    serde_json::to_string_pretty(&v).map_err(|e| format!("marshal result: {e}"))
}
