//! Reading the behaviour files of one eval scenario folder: `behaviour.yaml`
//! plus `traces/old.json` and `traces/new.json`. Every problem is collected, so
//! one run names everything that is wrong, and none of it touches a score.

use std::path::Path;

use super::scenario::Behaviour;
use super::trace::Trace;

/// A scenario's behaviour file with the two traces it compares.
pub struct Loaded {
    pub behaviour: Behaviour,
    /// The run recorded with the previous version of the skill.
    pub old: Trace,
    /// The run recorded with the current version of the skill.
    pub new: Trace,
}

fn read(path: &Path, label: &str) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| format!("{label}: cannot read: {e}"))
}

fn load_trace(dir: &Path, which: &str) -> Result<Trace, String> {
    let label = format!("traces/{which}.json");
    let path = dir.join(&label);
    if !path.is_file() {
        return Err(format!("{label} is missing"));
    }
    let text = read(&path, &label)?;
    Trace::parse(&text).map_err(|e| format!("{label}: {e}"))
}

/// Load the behaviour files of a scenario folder. `Ok(None)` means the scenario
/// has none, which is the normal case. `Err` lists every problem found.
pub fn load(dir: &Path) -> Result<Option<Loaded>, Vec<String>> {
    let yaml_path = dir.join("behaviour.yaml");
    if !yaml_path.is_file() {
        return if dir.join("traces").is_dir() {
            Err(vec!["traces/ is present without behaviour.yaml".to_string()])
        } else {
            Ok(None)
        };
    }

    let mut problems = Vec::new();
    let behaviour = read(&yaml_path, "behaviour.yaml")
        .and_then(|text| Behaviour::parse(&text).map_err(|e| format!("behaviour.yaml: {e}")));
    let old = load_trace(dir, "old");
    let new = load_trace(dir, "new");

    match (behaviour, old, new) {
        (Ok(behaviour), Ok(old), Ok(new)) => Ok(Some(Loaded {
            behaviour,
            old,
            new,
        })),
        (b, o, n) => {
            problems.extend(b.err());
            problems.extend(o.err());
            problems.extend(n.err());
            Err(problems)
        }
    }
}

/// What is wrong with a scenario's behaviour files, in plain sentences. Empty
/// when there are none or they are all fine.
pub fn problems(dir: &Path) -> Vec<String> {
    load(dir).err().unwrap_or_default()
}
