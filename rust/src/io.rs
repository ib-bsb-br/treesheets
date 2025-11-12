use std::fs;
use std::path::{Path, PathBuf};

use crate::sheet::Sheet;
use serde_json::Value;
use thiserror::Error;

/// Specialized errors produced while loading or saving sheets.
#[derive(Debug, Error)]
pub enum SheetIoError {
    #[error("failed to read sheet from {path}: {source}")]
    Read {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to parse sheet JSON from {path}: {source}")]
    Parse {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("failed to serialize sheet to {path}: {source}")]
    Serialize {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("failed to write sheet to {path}: {source}")]
    Write {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

pub fn load_sheet(path: impl AsRef<Path>) -> Result<Sheet, SheetIoError> {
    let path_ref = path.as_ref();
    let input = fs::read_to_string(path_ref).map_err(|source| SheetIoError::Read {
        path: path_ref.to_path_buf(),
        source,
    })?;

    serde_json::from_str(&input).map_err(|source| SheetIoError::Parse {
        path: path_ref.to_path_buf(),
        source,
    })
}

pub fn save_sheet(path: impl AsRef<Path>, sheet: &Sheet) -> Result<(), SheetIoError> {
    let path_ref = path.as_ref();
    let serialized =
        serde_json::to_string_pretty(sheet).map_err(|source| SheetIoError::Serialize {
            path: path_ref.to_path_buf(),
            source,
        })?;

    fs::write(path_ref, serialized).map_err(|source| SheetIoError::Write {
        path: path_ref.to_path_buf(),
        source,
    })
}

/// Performs a lenient validation pass ensuring the JSON payload follows the
/// schema expected by the application. The function intentionally keeps the
/// validation light-weight to support gradual feature parity development.
pub fn validate_sheet_json(value: &Value) -> bool {
    match value {
        Value::Object(map) => {
            let Some(title) = map.get("title") else {
                return false;
            };
            let Some(root) = map.get("root") else {
                return false;
            };
            title.is_string() && validate_cell(root)
        }
        _ => false,
    }
}

fn validate_cell(value: &Value) -> bool {
    match value {
        Value::Object(map) => {
            let Some(text) = map.get("text") else {
                return false;
            };
            if !text.is_string() {
                return false;
            }
            match map.get("children") {
                None => true,
                Some(Value::Array(children)) => children.iter().all(validate_cell),
                _ => false,
            }
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sheet::Sheet;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn sheet_round_trip_json() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join("sheet.json");
        let sheet = Sheet::sample();

        save_sheet(&path, &sheet).expect("serialize");
        let loaded = load_sheet(&path).expect("deserialize");

        assert_eq!(sheet, loaded);
    }

    #[test]
    fn json_validation_detects_missing_fields() {
        let invalid = json!({
            "title": "ok",
            "root": {
                "text": 5
            }
        });
        assert!(!validate_sheet_json(&invalid));

        let valid = serde_json::to_value(Sheet::sample()).expect("serialize");
        assert!(validate_sheet_json(&valid));
    }
}
