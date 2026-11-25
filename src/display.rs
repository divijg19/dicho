use crate::content::Entry;

pub fn print_entry(entry: &Entry, heading: &str) {
    println!("{heading}");
    println!();
    println!("  {}", entry.es);
    println!("  {}", entry.en);
    println!();
    println!("  Category: {}", entry.category.as_str());
    if let Some(author) = &entry.author {
        println!("  Author: {author}");
    }
    println!("  Source: {}", entry.source);
    println!("  License: {}", entry.license);
    if !entry.tags.is_empty() {
        println!("  Tags: {}", entry.tags.join(", "));
    }

    if let Some(featured) = entry.vocabulary.first() {
        println!();
        println!(
            "  Featured vocabulary: {} ({})",
            featured.term, featured.meaning
        );
    }

    if entry.translation_status == crate::content::TranslationStatus::Draft {
        println!();
        println!("  Translation status: draft, awaiting review");
    }
}
