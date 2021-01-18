//! Error types

use semver::{SemVerError, Version};
use serde_json::Error as JsonError;
use serde_yaml::Error as YamlError;
use std::io::Error as IoError;
use thiserror::Error;

/// errors that openapi functions may return
#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] IoError),
    #[error("YAML serialization or deserialization error")]
    Yaml(#[from] YamlError),
    #[error("JSON serialization error")]
    Serialize(#[from] JsonError),
    #[error("Semantic Versioning parsing error")]
    SemVerError(#[from] SemVerError),
    #[error("Unsupported spec file version ({0})")]
    UnsupportedSpecFileVersion(Version),
}
