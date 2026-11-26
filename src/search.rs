use std::collections::HashSet;

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

struct Doc {
    text: String,
    words: HashSet<String>,
}

/// Folded search data built once from the collection.
pub struct SearchIndex {
    docs: Vec<Doc>,
}

impl SearchIndex {
    pub fn new(entries: &[Entry]) -> Self {
        let docs = entries
            .iter()
            .map(|entry| {
                let text = haystack(entry);
                let words = text
                    .split(|character: char| !character.is_alphanumeric())
                    .filter(|word| !word.is_empty())
                    .map(str::to_owned)
                    .collect();
                Doc { text, words }
            })
            .collect();
        Self { docs }
    }

    pub fn matches(&self, index: usize, query: &Query) -> bool {
        let Some(doc) = self.docs.get(index) else {
            return false;
        };
        if query.substring {
            query.terms.iter().all(|term| doc.text.contains(term))
        } else {
            query.terms.iter().all(|term| doc.words.contains(term))
        }
    }
}

/// A folded query with whole word matching by default.
pub struct Query {
    terms: Vec<String>,
    substring: bool,
}

impl Query {
    pub fn parse(text: &str, substring: bool) -> Self {
        Self {
            terms: text
                .split_whitespace()
                .map(fold)
                .filter(|term| !term.is_empty())
                .collect(),
            substring,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.terms.is_empty()
    }
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
        let index = SearchIndex::new(&collection);
        let query = Query::parse("abarca", false);
        let found: Vec<&str> = collection
            .iter()
            .enumerate()
            .filter(|(position, _)| index.matches(*position, &query))
            .map(|(_, entry)| entry.id.as_str())
            .collect();
        assert!(found.contains(&"refran-010"));
    }

    #[test]
    fn search_matches_english_translation() {
        let collection = entries();
        let index = SearchIndex::new(&collection);
        let query = Query::parse("cobbler", false);
        assert!(index.matches(34, &query));
    }

    #[test]
    fn search_distinguishes_words_from_substrings() {
        let collection = entries();
        let index = SearchIndex::new(&collection);
        let word = Query::parse("clavo", false);
        let fragment = Query::parse("clav", false);
        let substring = Query::parse("clav", true);
        assert!(index.matches(27, &word));
        assert!(!index.matches(27, &fragment));
        assert!(index.matches(27, &substring));
    }

    #[test]
    fn multi_term_queries_require_all_terms() {
        let collection = entries();
        let index = SearchIndex::new(&collection);
        let both = Query::parse("clavo saca", false);
        let one = Query::parse("clavo missing", false);
        assert!(index.matches(27, &both));
        assert!(!index.matches(27, &one));
    }

    #[test]
    fn search_can_find_no_entries() {
        let collection = entries();
        let index = SearchIndex::new(&collection);
        let query = Query::parse("notpresent", false);
        assert!(
            collection
                .iter()
                .enumerate()
                .all(|(position, _)| !index.matches(position, &query))
        );
    }
}
