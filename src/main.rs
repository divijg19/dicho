mod browse;
mod cli;
mod content;
mod daily;
mod display;
mod filter;
mod history;
mod id;
mod search;
mod storage;
mod vocab;

use anyhow::{Result, anyhow};
use clap::Parser;
use cli::{Cli, Command, VocabCommand};

use crate::content::Entry;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let entries = content::load()?;
    let command = cli.command.unwrap_or(Command::Daily);

    match command {
        Command::Daily | Command::Repeat => {
            let date = daily::today()?;
            let entry = daily::daily(&entries, &date);
            display::print_entry(entry, "Daily Spanish");
            history::record_daily_if_new(&date, &entry.id)?;
        }
        Command::Random => {
            let entry = daily::random(&entries, &daily::today()?);
            display::print_entry(entry, "Random Spanish");
        }
        Command::Browse { filters } => {
            let found = filter::apply(&entries, &filters)?;
            print_lines(browse::browse_lines(&found));
        }
        Command::Show { id } => {
            display::print_detail(find_entry(&entries, &id)?);
        }
        Command::Categories => {
            print_lines(browse::category_lines(&entries));
        }
        Command::Tags => {
            print_lines(browse::tag_lines(&entries));
        }
        Command::Search {
            query,
            substring,
            filters,
        } => {
            let query = search::Query::parse(&query.join(" "), substring);
            if query.is_empty() {
                return Err(anyhow!("search query must not be empty"));
            }
            let index = search::SearchIndex::new(&entries);
            let found: Vec<Entry> = entries
                .iter()
                .enumerate()
                .filter(|(position, _)| index.matches(*position, &query))
                .map(|(_, entry)| entry.clone())
                .collect();
            let found = filter::apply(&found, &filters)?;
            print_lines(browse::browse_lines(&found));
        }
        Command::Save { id } => {
            find_entry(&entries, &id)?;
            let store = storage::Store::new(storage::Store::default_path()?);
            let mut ids = store.load()?;
            if !ids.contains(&id) {
                ids.push(id.clone());
                store.save(&ids)?;
                history::append(history::EventKind::Save, &id)?;
            }
            println!("Saved {id}");
        }
        Command::Unsave { id } => {
            let store = storage::Store::new(storage::Store::default_path()?);
            let mut ids = store.load()?;
            if let Some(position) = ids.iter().position(|saved| saved == &id) {
                ids.remove(position);
                store.save(&ids)?;
                history::append(history::EventKind::Unsave, &id)?;
                println!("Removed {id}");
            }
        }
        Command::Saved => {
            let store = storage::Store::new(storage::Store::default_path()?);
            let ids = store.load()?;
            if ids.is_empty() {
                println!("No saved entries.");
            } else {
                let found: Vec<Entry> = ids
                    .iter()
                    .filter_map(|id| entries.iter().find(|entry| entry.id == *id))
                    .cloned()
                    .collect();
                print_lines(browse::browse_lines(&found));
            }
        }
        Command::History { limit } => {
            let events = history::recent(limit)?;
            if events.is_empty() {
                println!("No history yet.");
            } else {
                print_lines(history::lines(&events));
            }
        }
        Command::Vocab { command } => {
            let store = vocab::VocabStore::new(vocab::VocabStore::default_path()?);
            let mut items = store.load()?;

            match command {
                VocabCommand::Add {
                    term,
                    meaning,
                    example,
                    notes,
                    entry_ids,
                } => {
                    for id in &entry_ids {
                        find_entry(&entries, id)?;
                    }
                    let item = vocab::VocabItem::new(
                        term,
                        meaning,
                        example,
                        notes,
                        entry_ids,
                        daily::timestamp()?,
                    );
                    if let Some(position) = items.iter().position(|existing| existing.id == item.id)
                    {
                        let mut changed = false;
                        for id in &item.entry_ids {
                            if !items[position].entry_ids.contains(id) {
                                items[position].entry_ids.push(id.clone());
                                changed = true;
                            }
                        }
                        if items[position].example.is_none() && item.example.is_some() {
                            items[position].example = item.example.clone();
                            changed = true;
                        }
                        if items[position].notes.is_none() && item.notes.is_some() {
                            items[position].notes = item.notes.clone();
                            changed = true;
                        }
                        if changed {
                            let id = item.id.clone();
                            store.save(&items)?;
                            println!("Updated {id}");
                        } else {
                            println!("Already have {}", item.id);
                        }
                    } else {
                        let id = item.id.clone();
                        items.push(item);
                        store.save(&items)?;
                        println!("Added {id}");
                    }
                }
                VocabCommand::List {
                    search,
                    saved,
                    entry,
                } => {
                    let mut shown: Vec<&vocab::VocabItem> = items.iter().collect();
                    if let Some(query) = search {
                        shown.retain(|item| vocab::matches(item, &query));
                    }
                    if saved {
                        shown.retain(|item| item.saved);
                    }
                    if let Some(entry) = entry {
                        shown.retain(|item| item.entry_ids.iter().any(|id| id == &entry));
                    }
                    if shown.is_empty() {
                        println!("No vocabulary items.");
                    } else {
                        print_lines(shown.into_iter().map(vocab::line));
                    }
                }
                VocabCommand::Search { query } => {
                    let found: Vec<String> = items
                        .iter()
                        .filter(|item| vocab::matches(item, &query))
                        .map(vocab::line)
                        .collect();
                    if found.is_empty() {
                        println!("No vocabulary items.");
                    } else {
                        print_lines(found);
                    }
                }
                VocabCommand::Show { id } => {
                    let item = items
                        .iter()
                        .find(|item| item.id == id)
                        .ok_or_else(|| anyhow!("no vocabulary item with id '{id}'"))?;
                    print_lines(vocab::detail_lines(item));
                }
                VocabCommand::Remove { id } => {
                    if let Some(position) = items.iter().position(|item| item.id == id) {
                        items.remove(position);
                        store.save(&items)?;
                        println!("Removed {id}");
                    }
                }
                VocabCommand::Save { id } => {
                    if let Some(position) = items.iter().position(|item| item.id == id)
                        && !items[position].saved
                    {
                        items[position].saved = true;
                        store.save(&items)?;
                        println!("Saved {id}");
                    }
                }
                VocabCommand::Unsave { id } => {
                    if let Some(position) = items.iter().position(|item| item.id == id)
                        && items[position].saved
                    {
                        items[position].saved = false;
                        store.save(&items)?;
                        println!("Unsaved {id}");
                    }
                }
            }
        }
    }

    Ok(())
}

fn find_entry<'a>(entries: &'a [Entry], id: &str) -> Result<&'a Entry> {
    entries
        .iter()
        .find(|entry| entry.id == id)
        .ok_or_else(|| anyhow!("no entry with id '{id}'"))
}

fn print_lines(lines: impl IntoIterator<Item = String>) {
    for line in lines {
        println!("{line}");
    }
}
