//! Shared error type for the tekhne tool crates.

use std::path::{Path, PathBuf};

/// Errors shared across the tekhne tool crates.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An i/o operation failed; carries the path for context.
    #[error("i/o error at {path}: {source}")]
    Io {
        /// The path the operation targeted.
        path: PathBuf,
        /// The underlying i/o error.
        #[source]
        source: std::io::Error,
    },

    /// A JSON (de)serialization failed; `context` names what was being processed.
    #[error("json error in {context}: {source}")]
    Json {
        /// A human-readable description of what was being (de)serialized.
        context: String,
        /// The underlying serde_json error.
        #[source]
        source: serde_json::Error,
    },

    /// A required path did not exist.
    #[error("not found: {0}")]
    NotFound(PathBuf),

    /// A configuration value was invalid or missing.
    #[error("config error: {0}")]
    Config(String),
}

impl Error {
    /// Build an [`Error::Io`] from a path and the underlying i/o error.
    pub fn io(path: impl AsRef<Path>, source: std::io::Error) -> Self {
        Error::Io {
            path: path.as_ref().to_path_buf(),
            source,
        }
    }

    /// Build an [`Error::Json`] from a context label and the underlying error.
    pub fn json(context: impl Into<String>, source: serde_json::Error) -> Self {
        Error::Json {
            context: context.into(),
            source,
        }
    }
}

/// Convenience result alias used throughout the tool crates.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn io_error_includes_path() {
        let src = std::io::Error::new(std::io::ErrorKind::NotFound, "nope");
        let err = Error::io("/tmp/x", src);
        assert!(err.to_string().contains("/tmp/x"));
    }

    #[test]
    fn config_error_message() {
        assert_eq!(
            Error::Config("bad value".into()).to_string(),
            "config error: bad value"
        );
    }
}
