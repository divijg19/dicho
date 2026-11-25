use anyhow::{Context, Result};
use serde::Deserialize;

const RAW_ENTRIES: &str = include_str!("../data/curated/entries.jsonl");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Proverb,
    Saying,
}

impl Category {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Proverb => "proverb",
            Self::Saying => "saying",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TranslationStatus {
    Draft,
    Reviewed,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Vocabulary {
    pub term: String,
    pub meaning: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Entry {
    pub id: String,
    pub es: String,
    pub en: String,
    pub category: Category,
    #[serde(default)]
    pub author: Option<String>,
    pub source: String,
    pub license: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub translation_status: TranslationStatus,
    #[serde(default)]
    pub vocabulary: Vec<Vocabulary>,
}

/// Load the embedded collection, ordered by stable identifier.
///
/// Sorting here keeps daily selection stable even if the source lines are
/// reordered.
pub fn load() -> Result<Vec<Entry>> {
    let mut entries = Vec::new();

    for (index, line) in RAW_ENTRIES.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let entry: Entry = serde_json::from_str(line)
            .with_context(|| format!("invalid curated entry on line {}", index + 1))?;
        entries.push(entry);
    }

    entries.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collection_is_well_formed() {
        let entries = load().expect("embedded collection should parse");
        assert_eq!(entries.len(), 20);

        for entry in &entries {
            assert!(!entry.id.is_empty());
            assert!(!entry.es.is_empty());
            assert!(!entry.en.is_empty());
            assert!(!entry.source.is_empty());
            assert!(!entry.license.is_empty());
        }
    }

    #[test]
    fn identifiers_are_unique_and_sorted() {
        let entries = load().expect("embedded collection should parse");
        let mut ids: Vec<&str> = entries.iter().map(|entry| entry.id.as_str()).collect();
        ids.dedup();
        assert_eq!(ids.len(), entries.len());
        assert!(entries.windows(2).all(|pair| pair[0].id <= pair[1].id));
    }

    #[test]
    fn every_entry_has_featured_vocabulary() {
        let entries = load().expect("embedded collection should parse");
        assert!(entries.iter().all(|entry| !entry.vocabulary.is_empty()));
    }
}
