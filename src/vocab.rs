use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::id::stable_id;
use crate::search::fold;
use crate::storage::{Store as DataStore, write_atomic};

const STATE_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VocabItem {
    pub id: String,
    pub term: String,
    pub meaning: String,
    pub example: Option<String>,
    pub notes: Option<String>,
    pub entry_ids: Vec<String>,
    pub saved: bool,
    pub created_at: String,
}

impl VocabItem {
    pub fn new(
        term: String,
        meaning: String,
        example: Option<String>,
        notes: Option<String>,
        entry_ids: Vec<String>,
        created_at: String,
    ) -> Self {
        let identity = format!("{}|{}", fold(&term), fold(&meaning));
        Self {
            id: stable_id("vocab", &identity),
            term,
            meaning,
            example,
            notes,
            entry_ids,
            saved: false,
            created_at,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct VocabState {
    version: u32,
    items: Vec<VocabItem>,
}

pub struct VocabStore {
    path: PathBuf,
}

impl VocabStore {
    pub fn default_path() -> Result<PathBuf> {
        Ok(DataStore::data_dir()?.join("vocabulary.json"))
    }

    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn load(&self) -> Result<Vec<VocabItem>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }
        let text = fs::read_to_string(&self.path)
            .with_context(|| format!("cannot read {}", self.path.display()))?;
        let state: VocabState = serde_json::from_str(&text)
            .with_context(|| format!("invalid vocabulary data in {}", self.path.display()))?;
        if state.version != STATE_VERSION {
            bail!(
                "unsupported vocabulary data version {} in {}",
                state.version,
                self.path.display()
            );
        }

        let mut seen = HashSet::new();
        Ok(state
            .items
            .into_iter()
            .filter(|item| seen.insert(item.id.clone()))
            .collect())
    }

    pub fn save(&self, items: &[VocabItem]) -> Result<()> {
        let state = VocabState {
            version: STATE_VERSION,
            items: items.to_vec(),
        };
        let text = serde_json::to_string_pretty(&state)?;
        write_atomic(&self.path, &format!("{text}\n"))
    }
}

pub fn matches(item: &VocabItem, query: &str) -> bool {
    let needle = fold(query);
    let mut fields = vec![item.term.as_str(), item.meaning.as_str()];
    if let Some(example) = &item.example {
        fields.push(example);
    }
    if let Some(notes) = &item.notes {
        fields.push(notes);
    }
    fields.extend(item.entry_ids.iter().map(String::as_str));
    fields.iter().any(|field| fold(field).contains(&needle))
}

pub fn line(item: &VocabItem) -> String {
    let suffix = if item.saved { "  [saved]" } else { "" };
    format!("{}  {}  {}{}", item.id, item.term, item.meaning, suffix)
}

pub fn detail_lines(item: &VocabItem) -> Vec<String> {
    let mut lines = vec![
        format!("Vocabulary: {}", item.id),
        String::new(),
        format!("  Term: {}", item.term),
        format!("  Meaning: {}", item.meaning),
    ];
    if let Some(example) = &item.example {
        lines.push(format!("  Example: {example}"));
    }
    if let Some(notes) = &item.notes {
        lines.push(format!("  Notes: {notes}"));
    }
    if !item.entry_ids.is_empty() {
        lines.push(format!("  Entries: {}", item.entry_ids.join(", ")));
    }
    lines.push(format!("  Added: {}", item.created_at));
    lines.push(format!("  Saved: {}", item.saved));
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item() -> VocabItem {
        VocabItem::new(
            "Ábarca".to_owned(),
            "grasps".to_owned(),
            None,
            None,
            vec!["refran-010".to_owned()],
            "2026-11-28T10:00:00Z".to_owned(),
        )
    }

    #[test]
    fn identifier_uses_folded_term_and_meaning() {
        assert_eq!(item().id, stable_id("vocab", "abarca|grasps"));
        let same = VocabItem::new(
            "abarca".to_owned(),
            "grasps".to_owned(),
            None,
            None,
            vec![],
            "t".to_owned(),
        );
        assert_eq!(item().id, same.id);
        let other_sense = VocabItem::new(
            "abarca".to_owned(),
            "overreach".to_owned(),
            None,
            None,
            vec![],
            "t".to_owned(),
        );
        assert_ne!(item().id, other_sense.id);
    }

    #[test]
    fn search_is_accent_insensitive() {
        assert!(matches(&item(), "abarca"));
        assert!(matches(&item(), "ABARCA"));
        assert!(matches(&item(), "refran-010"));
        assert!(!matches(&item(), "missing"));
    }

    #[test]
    fn save_and_load_round_trip() {
        let directory =
            std::env::temp_dir().join(format!("dicho-vocab-test-{}", std::process::id()));
        let store = VocabStore::new(directory.join("vocabulary.json"));
        let items = vec![item()];
        store.save(&items).expect("save should succeed");
        assert_eq!(store.load().expect("load should succeed"), items);
        fs::remove_dir_all(&directory).ok();
    }

    #[test]
    fn load_rejects_unknown_version() {
        let directory =
            std::env::temp_dir().join(format!("dicho-vocab-version-{}", std::process::id()));
        let path = directory.join("vocabulary.json");
        fs::create_dir_all(&directory).expect("directory should be created");
        fs::write(&path, "{\"version\":9,\"items\":[]}").expect("fixture should be written");
        assert!(VocabStore::new(path).load().is_err());
        fs::remove_dir_all(&directory).ok();
    }
}
