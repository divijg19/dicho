use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

const STATE_VERSION: u32 = 1;

#[derive(Debug, Serialize, Deserialize)]
struct SavedState {
    version: u32,
    ids: Vec<String>,
}

pub struct Store {
    path: PathBuf,
}

impl Store {
    /// Resolve the data directory from a project override, XDG, or the home directory.
    pub fn data_dir() -> Result<PathBuf> {
        if let Some(dir) = std::env::var_os("DICHO_DATA_DIR")
            && !dir.is_empty()
        {
            return Ok(PathBuf::from(dir));
        }
        if let Some(dir) = std::env::var_os("XDG_DATA_HOME")
            && !dir.is_empty()
        {
            return Ok(PathBuf::from(dir).join("dicho"));
        }
        let home = std::env::var_os("HOME").context("HOME is not set")?;
        Ok(PathBuf::from(home)
            .join(".local")
            .join("share")
            .join("dicho"))
    }

    /// Resolve the path of the saved entries file.
    pub fn default_path() -> Result<PathBuf> {
        Ok(Self::data_dir()?.join("saved.json"))
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
        if state.version != STATE_VERSION {
            bail!(
                "unsupported saved data version {} in {}",
                state.version,
                self.path.display()
            );
        }

        let mut seen = HashSet::new();
        Ok(state
            .ids
            .into_iter()
            .filter(|id| seen.insert(id.clone()))
            .collect())
    }

    pub fn save(&self, ids: &[String]) -> Result<()> {
        let parent = self
            .path
            .parent()
            .context("saved data path has no parent directory")?;
        fs::create_dir_all(parent)?;

        let state = SavedState {
            version: STATE_VERSION,
            ids: ids.to_vec(),
        };
        let text = serde_json::to_string_pretty(&state)?;
        let file_name = self
            .path
            .file_name()
            .context("saved data path has no file name")?
            .to_string_lossy();
        let temporary = parent.join(format!(".{file_name}.{}.tmp", std::process::id()));
        fs::write(&temporary, format!("{text}\n"))
            .with_context(|| format!("cannot write {}", temporary.display()))?;
        fs::rename(&temporary, &self.path)
            .with_context(|| format!("cannot replace {}", self.path.display()))?;
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

    #[test]
    fn load_deduplicates_ids() {
        let dir = test_dir();
        let store = Store::new(dir.join("saved.json"));
        let ids = vec!["refran-001".to_owned(), "refran-001".to_owned()];
        store.save(&ids).expect("save should succeed");
        assert_eq!(store.load().expect("load should succeed").len(), 1);
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn load_rejects_unknown_version() {
        let dir = test_dir();
        let path = dir.join("saved.json");
        fs::create_dir_all(&dir).expect("directory should be created");
        fs::write(&path, "{\"version\":9,\"ids\":[]}").expect("fixture should be written");
        let store = Store::new(path);
        assert!(store.load().is_err());
        fs::remove_dir_all(&dir).ok();
    }
}
