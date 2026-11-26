use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct SavedState {
    version: u32,
    ids: Vec<String>,
}

pub struct Store {
    path: PathBuf,
}

impl Store {
    /// Resolve the data path from a project override, XDG, or the home directory.
    pub fn default_path() -> Result<PathBuf> {
        if let Some(dir) = std::env::var_os("DICHO_DATA_DIR") {
            if !dir.is_empty() {
                return Ok(PathBuf::from(dir).join("saved.json"));
            }
        }
        if let Some(dir) = std::env::var_os("XDG_DATA_HOME") {
            if !dir.is_empty() {
                return Ok(PathBuf::from(dir).join("dicho").join("saved.json"));
            }
        }
        let home = std::env::var_os("HOME").context("HOME is not set")?;
        Ok(PathBuf::from(home)
            .join(".local")
            .join("share")
            .join("dicho")
            .join("saved.json"))
    }

    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn load(&self) -> Result<Vec<String>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }
        let text = fs::read_to_string(&self.path)
            .with_context(|| format!("cannot read {}", self.path.display()))?;
        let state: SavedState = serde_json::from_str(&text)
            .with_context(|| format!("invalid saved data in {}", self.path.display()))?;
        Ok(state.ids)
    }

    pub fn save(&self, ids: &[String]) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let state = SavedState {
            version: 1,
            ids: ids.to_vec(),
        };
        let text = serde_json::to_string_pretty(&state)?;
        fs::write(&self.path, format!("{text}\n"))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_dir() -> PathBuf {
        std::env::temp_dir().join(format!("dicho-store-test-{}", std::process::id()))
    }

    #[test]
    fn save_and_load_round_trip() {
        let dir = test_dir();
        let store = Store::new(dir.join("saved.json"));
        let ids = vec!["refran-001".to_owned(), "refran-040".to_owned()];
        store.save(&ids).expect("save should succeed");
        assert_eq!(store.load().expect("load should succeed"), ids);
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn missing_file_loads_as_empty() {
        let store = Store::new(test_dir().join("missing.json"));
        assert!(store.load().expect("load should succeed").is_empty());
    }
}
