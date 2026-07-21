//! Configuration loading and environment overrides.

use std::path::Path;

use serde::de::DeserializeOwned;

use crate::error::{Error, Result};
use crate::fs::read_to_string;

/// Load a JSON config file and deserialize it into `T`.
///
/// Returns [`Error::NotFound`] when the file is absent and [`Error::Json`]
/// when the contents fail to parse.
pub fn load_json<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    let path = path.as_ref();
    if !path.exists() {
        return Err(Error::NotFound(path.to_path_buf()));
    }
    let text = read_to_string(path)?;
    serde_json::from_str(&text).map_err(|source| Error::json(path.display().to_string(), source))
}

/// Read an environment variable, treating unset or empty as absent.
///
/// Tool crates use this for env overrides (e.g. `CLAUDE_CONFIG_DIR`).
pub fn env_override(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|v| !v.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Cfg {
        name: String,
        count: u32,
    }

    #[test]
    fn load_json_roundtrip() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("cfg.json");
        std::fs::write(&path, r#"{"name":"tekhne","count":3}"#).unwrap();
        let cfg: Cfg = load_json(&path).unwrap();
        assert_eq!(
            cfg,
            Cfg {
                name: "tekhne".into(),
                count: 3
            }
        );
    }

    #[test]
    fn load_json_missing_is_not_found() {
        let err = load_json::<Cfg>("/no/such/cfg.json").unwrap_err();
        assert!(matches!(err, Error::NotFound(_)));
    }

    #[test]
    fn load_json_invalid_is_json_error() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("bad.json");
        std::fs::write(&path, "{ not json").unwrap();
        let err = load_json::<Cfg>(&path).unwrap_err();
        assert!(matches!(err, Error::Json { .. }));
    }

    #[test]
    fn env_override_ignores_empty() {
        // A name unlikely to be set in the environment.
        let key = "TEKHNE_COMMON_TEST_ENV_UNSET";
        assert_eq!(env_override(key), None);
    }
}
