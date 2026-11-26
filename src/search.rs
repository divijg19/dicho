use crate::content::Entry;

/// Lowercase text and fold Spanish diacritics to their base letters.
pub fn fold(text: &str) -> String {
    let mut folded = String::new();
    for character in text.chars() {
        for lower in character.to_lowercase() {
            match lower {
                'á' | 'à' | 'ä' | 'â' | 'ã' | 'å' => folded.push('a'),
                'é' | 'è' | 'ë' | 'ê' | 'ẽ' => folded.push('e'),
                'í' | 'ì' | 'ï' | 'î' => folded.push('i'),
                'ó' | 'ò' | 'ö' | 'ô' | 'õ' => folded.push('o'),
                'ú' | 'ù' | 'ü' | 'û' => folded.push('u'),
                'ñ' => folded.push('n'),
                'ç' => folded.push('c'),
                'ý' | 'ÿ' => folded.push('y'),
                '¿' | '¡' => {}
                _ => folded.push(lower),
            }
        }
    }
    folded
}

fn haystack(entry: &Entry) -> String {
    let mut parts = vec![entry.es.clone(), entry.en.clone(), entry.source.clone()];
    if let Some(author) = &entry.author {
        parts.push(author.clone());
    }
    parts.extend(entry.tags.iter().cloned());
    for item in &entry.vocabulary {
        parts.push(item.term.clone());
        parts.push(item.meaning.clone());
    }
    fold(&parts.join(" "))
}

pub fn matches(entry: &Entry, query: &str) -> bool {
    haystack(entry).contains(&fold(query))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entries() -> Vec<Entry> {
        crate::content::load().expect("embedded collection should parse")
    }

    #[test]
    fn fold_handles_spanish_characters() {
        assert_eq!(fold("ÁBARCA"), "abarca");
        assert_eq!(fold("Corazón"), "corazon");
        assert_eq!(fold("Dime con quién andas"), "dime con quien andas");
        assert_eq!(fold("¿Qué tal?"), "que tal?");
    }

    #[test]
    fn search_matches_spanish_text_without_accents() {
        let collection = entries();
        let found: Vec<&str> = collection
            .iter()
            .filter(|entry| matches(entry, "abarca"))
            .map(|entry| entry.id.as_str())
            .collect();
        assert!(found.contains(&"refran-010"));
    }

    #[test]
    fn search_matches_english_translation() {
        let collection = entries();
        let found: Vec<&str> = collection
            .iter()
            .filter(|entry| matches(entry, "cobbler"))
            .map(|entry| entry.id.as_str())
            .collect();
        assert!(found.contains(&"refran-035"));
    }

    #[test]
    fn search_matches_tags_and_source() {
        assert!(
            entries()
                .iter()
                .any(|entry| matches(entry, "sbarbi") && entry.id == "refran-021")
        );
        assert!(
            entries()
                .iter()
                .any(|entry| matches(entry, "optimism") && entry.id == "refran-011")
        );
    }

    #[test]
    fn search_can_find_no_entries() {
        assert!(entries().iter().all(|entry| !matches(entry, "notpresent")));
    }
}
