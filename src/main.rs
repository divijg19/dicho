mod browse;
mod cli;
mod content;
mod daily;
mod display;
mod filter;
mod history;
mod search;
mod storage;

use anyhow::{Result, anyhow};
use clap::Parser;
use cli::{Cli, Command};

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
            let ids: Vec<String> = store
                .load()?
                .into_iter()
                .filter(|saved| saved != &id)
                .collect();
            store.save(&ids)?;
            history::append(history::EventKind::Unsave, &id)?;
            println!("Removed {id}");
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
