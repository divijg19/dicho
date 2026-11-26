use std::collections::BTreeMap;

use crate::content::Entry;

pub fn browse_lines(entries: &[Entry]) -> Vec<String> {
    entries
        .iter()
        .map(|entry| {
            let difficulty = entry
                .difficulty
                .map(|value| format!("{value:.2}"))
                .unwrap_or_else(|| "n/a".to_owned());
            format!(
                "{}  {}  [{}]  d={difficulty}",
                entry.id,
                entry.es,
                entry.category.as_str()
            )
        })
        .collect()
}

pub fn category_lines(entries: &[Entry]) -> Vec<String> {
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for entry in entries {
        *counts.entry(entry.category.as_str()).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .map(|(name, count)| format!("{name}  {count}"))
        .collect()
}

pub fn tag_lines(entries: &[Entry]) -> Vec<String> {
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for entry in entries {
        for tag in &entry.tags {
            *counts.entry(tag.as_str()).or_insert(0) += 1;
        }
    }
    counts
        .into_iter()
        .map(|(name, count)| format!("{name}  {count}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entries() -> Vec<Entry> {
        crate::content::load().expect("embedded collection should parse")
    }

    #[test]
    fn browse_lines_cover_every_entry() {
        let lines = browse_lines(&entries());
        assert_eq!(lines.len(), 40);
        assert!(lines[0].starts_with("refran-001  "));
        assert!(lines[0].contains("[proverb]"));
    }

    #[test]
    fn category_counts_are_sorted() {
        let lines = category_lines(&entries());
        assert_eq!(lines.len(), 2);
        assert!(lines[0].starts_with("proverb  "));
        assert!(lines[1].starts_with("saying  "));
    }

    #[test]
    fn tag_counts_include_known_tags() {
        let lines = tag_lines(&entries());
        assert!(lines.iter().any(|line| line.starts_with("optimism  2")));
    }
}
