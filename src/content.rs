use std::collections::HashSet;

use anyhow::{Context, Result, anyhow, bail};
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

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "proverb" => Some(Self::Proverb),
            "saying" => Some(Self::Saying),
            _ => None,
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
    #[serde(default)]
    pub difficulty: Option<f32>,
    pub translation_status: TranslationStatus,
    #[serde(default)]
    pub vocabulary: Vec<Vocabulary>,
}

/// Load the embedded collection, ordered by stable identifier.
///
/// Sorting here keeps daily selection stable even if the source lines are
/// reordered.
pub fn load() -> Result<Vec<Entry>> {
    parse(RAW_ENTRIES)
}

fn parse(raw: &str) -> Result<Vec<Entry>> {
    let mut entries = Vec::new();
    let mut seen = HashSet::new();

    for (index, line) in raw.lines().enumerate() {
        let line_number = index + 1;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let entry: Entry = serde_json::from_str(line)
            .with_context(|| format!("invalid curated entry on line {line_number}"))?;
        validate(&entry, line_number)?;

        if !seen.insert(entry.id.clone()) {
            bail!("duplicate entry id '{}' on line {line_number}", entry.id);
        }
        entries.push(entry);
    }

    if entries.is_empty() {
        bail!("curated collection is empty");
    }

    entries.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(entries)
}

fn validate(entry: &Entry, line_number: usize) -> Result<()> {
    let required = |field: &str| anyhow!("line {line_number}: {field} must not be empty");

    if entry.id.trim().is_empty() {
        return Err(required("id"));
    }
    if entry.es.trim().is_empty() {
        return Err(required("es"));
    }
    if entry.en.trim().is_empty() {
        return Err(required("en"));
    }
    if entry.source.trim().is_empty() {
        return Err(required("source"));
    }
    if entry.license.trim().is_empty() {
        return Err(required("license"));
    }
    if let Some(difficulty) = entry.difficulty {
        if !(0.0..=1.0).contains(&difficulty) {
            bail!("line {line_number}: difficulty must be between 0.0 and 1.0");
        }
    }
    if entry.vocabulary.is_empty() {
        bail!("line {line_number}: vocabulary must not be empty");
    }
    for item in &entry.vocabulary {
        if item.term.trim().is_empty() {
            return Err(required("vocabulary term"));
        }
        if item.meaning.trim().is_empty() {
            return Err(required("vocabulary meaning"));
        }
    }
    for tag in &entry.tags {
        if tag.trim().is_empty() {
            return Err(required("tag"));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{"id":"test-001","es":"Hola.","en":"Hello.","category":"proverb","source":"Test","license":"Public domain","translation_status":"draft","vocabulary":[{"term":"hola","meaning":"hello"}]}"#;

    #[test]
    fn collection_is_well_formed() {
        let entries = load().expect("embedded collection should parse");
        assert_eq!(entries.len(), 40);
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

    #[test]
    fn parse_rejects_an_empty_collection() {
        let error = parse("").expect_err("empty input should fail");
        assert!(error.to_string().contains("empty"));
    }

    #[test]
    fn parse_rejects_duplicate_ids() {
        let raw = format!("{SAMPLE}\n{SAMPLE}");
        let error = parse(&raw).expect_err("duplicate ids should fail");
        assert!(error.to_string().contains("duplicate"));
    }

    #[test]
    fn parse_rejects_missing_required_fields() {
        let raw = SAMPLE.replace("\"es\":\"Hola.\",", "");
        let error = parse(&raw).expect_err("missing field should fail");
        assert!(error.to_string().contains("line 1"));
    }

    #[test]
    fn parse_rejects_difficulty_out_of_range() {
        let raw = SAMPLE.replace(
            "\"translation_status\"",
            "\"difficulty\":1.5,\"translation_status\"",
        );
        let error = parse(&raw).expect_err("invalid difficulty should fail");
        assert!(error.to_string().contains("difficulty"));
    }

    #[test]
    fn parse_rejects_blank_required_fields() {
        let raw = SAMPLE.replace("\"es\":\"Hola.\"", "\"es\":\"\"");
        let error = parse(&raw).expect_err("blank field should fail");
        assert!(error.to_string().contains("es must not be empty"));
    }
}
