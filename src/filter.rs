use anyhow::{Result, bail};

use crate::cli::Filters;
use crate::content::{Category, Entry};
use crate::search::fold;

pub fn apply(entries: &[Entry], filters: &Filters) -> Result<Vec<Entry>> {
    if let Some(min) = filters.min_difficulty
        && !(0.0..=1.0).contains(&min)
    {
        bail!("min difficulty must be between 0.0 and 1.0");
    }
    if let Some(max) = filters.max_difficulty
        && !(0.0..=1.0).contains(&max)
    {
        bail!("max difficulty must be between 0.0 and 1.0");
    }
    if let (Some(min), Some(max)) = (filters.min_difficulty, filters.max_difficulty)
        && min > max
    {
        bail!("min difficulty must not exceed max difficulty");
    }

    let category = match &filters.category {
        Some(value) => match Category::parse(&fold(value)) {
            Some(category) => Some(category),
            None => bail!("unknown category '{value}'"),
        },
        None => None,
    };
    let tag = filters.tag.as_ref().map(|value| fold(value));

    Ok(entries
        .iter()
        .filter(|entry| {
            if let Some(expected) = category
                && entry.category != expected
            {
                return false;
            }
            if let Some(expected) = &tag
                && !entry.tags.iter().any(|item| fold(item) == *expected)
            {
                return false;
            }
            if let Some(min) = filters.min_difficulty
                && entry.difficulty.is_none_or(|value| value < min)
            {
                return false;
            }
            if let Some(max) = filters.max_difficulty
                && entry.difficulty.is_none_or(|value| value > max)
            {
                return false;
            }
            true
        })
        .cloned()
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entries() -> Vec<Entry> {
        crate::content::load().expect("embedded collection should parse")
    }

    #[test]
    fn category_filter_selects_one_category() {
        let filters = Filters {
            category: Some("saying".to_owned()),
            ..Filters::default()
        };
        let found = apply(&entries(), &filters).expect("filter should succeed");
        assert_eq!(found.len(), 5);
        assert!(found.iter().all(|entry| entry.category == Category::Saying));
    }

    #[test]
    fn category_filter_is_case_insensitive() {
        let filters = Filters {
            category: Some("SAYING".to_owned()),
            ..Filters::default()
        };
        let found = apply(&entries(), &filters).expect("filter should succeed");
        assert_eq!(found.len(), 5);
    }

    #[test]
    fn tag_filter_is_case_insensitive() {
        let filters = Filters {
            tag: Some("OPTIMISM".to_owned()),
            ..Filters::default()
        };
        let found = apply(&entries(), &filters).expect("filter should succeed");
        assert_eq!(found.len(), 2);
    }

    #[test]
    fn difficulty_range_filters_entries() {
        let filters = Filters {
            min_difficulty: Some(0.40),
            max_difficulty: Some(0.55),
            ..Filters::default()
        };
        let found = apply(&entries(), &filters).expect("filter should succeed");
        assert!(found.iter().all(
            |entry| matches!(entry.difficulty, Some(value) if (0.40..=0.55).contains(&value))
        ));
        assert!(found.iter().any(|entry| entry.id == "refran-039"));
    }

    #[test]
    fn invalid_filters_are_rejected() {
        let bad_category = Filters {
            category: Some("unknown".to_owned()),
            ..Filters::default()
        };
        assert!(apply(&entries(), &bad_category).is_err());

        let bad_range = Filters {
            min_difficulty: Some(0.8),
            max_difficulty: Some(0.2),
            ..Filters::default()
        };
        assert!(apply(&entries(), &bad_range).is_err());
    }
}
