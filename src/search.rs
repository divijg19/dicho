use std::collections::HashSet;

use crate::content::Entry;

/// Lowercase text and fold Spanish diacritics to their base letters.
pub fn fold(text: &str) -> String {
    let mut folded = String::new();
    for character in text.chars() {
        for lower in character.to_lowercase() {
            if ('\u{0300}'..='\u{036f}').contains(&lower) {
                continue;
            }
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

/// Split folded text into whole word tokens.
fn words(text: &str) -> impl Iterator<Item = &str> + '_ {
    text.split(|character: char| !character.is_alphanumeric())
        .filter(|word| !word.is_empty())
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
                let word_set = words(&text).map(str::to_owned).collect();
                Doc {
                    text,
                    words: word_set,
                }
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
        let folded = fold(text);
        Self {
            terms: words(&folded).map(str::to_owned).collect(),
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

    fn position(collection: &[Entry], id: &str) -> usize {
        collection
            .iter()
            .position(|entry| entry.id == id)
            .unwrap_or_else(|| panic!("missing entry {id}"))
    }

    #[test]
    fn fold_handles_spanish_characters() {
        assert_eq!(fold("ÁBARCA"), "abarca");
        assert_eq!(fold("Corazón"), "corazon");
        assert_eq!(fold("Dime con quién andas"), "dime con quien andas");
        assert_eq!(fold("¿Qué tal?"), "que tal?");
    }

    #[test]
    fn fold_strips_combining_marks() {
        let decomposed = "a\u{0301}barca";
        assert_eq!(fold(decomposed), "abarca");
        let collection = entries();
        let index = SearchIndex::new(&collection);
        let query = Query::parse(decomposed, false);
        assert!(index.matches(position(&collection, "refran-010"), &query));
    }

    #[test]
    fn query_tokens_ignore_punctuation() {
        let query = Query::parse("Zapatero, a tus zapatos.", false);
        assert_eq!(query.terms, vec!["zapatero", "a", "tus", "zapatos"]);
    }

    #[test]
    fn punctuated_query_matches_whole_words() {
        let collection = entries();
        let index = SearchIndex::new(&collection);
        let query = Query::parse("Zapatero, a tus zapatos.", false);
        assert!(index.matches(position(&collection, "refran-035"), &query));
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
        assert!(index.matches(position(&collection, "refran-035"), &query));
    }

    #[test]
    fn search_distinguishes_words_from_substrings() {
        let collection = entries();
        let index = SearchIndex::new(&collection);
        let word = Query::parse("clavo", false);
        let fragment = Query::parse("clav", false);
        let substring = Query::parse("clav", true);
        let target = position(&collection, "refran-028");
        assert!(index.matches(target, &word));
        assert!(!index.matches(target, &fragment));
        assert!(index.matches(target, &substring));
    }

    #[test]
    fn multi_term_queries_require_all_terms() {
        let collection = entries();
        let index = SearchIndex::new(&collection);
        let both = Query::parse("clavo saca", false);
        let one = Query::parse("clavo missing", false);
        let target = position(&collection, "refran-028");
        assert!(index.matches(target, &both));
        assert!(!index.matches(target, &one));
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
