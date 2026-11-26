mod browse;
mod cli;
mod content;
mod daily;
mod display;
mod filter;
mod history;
mod search;
mod storage;

use anyhow::{Result, bail};
use clap::Parser;
use cli::{Cli, Command};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let entries = content::load()?;
    let command = cli.command.unwrap_or(Command::Daily);

    match command {
        Command::Daily => {
            let date = daily::today()?;
            let entry = daily::daily(&entries, &date);
            display::print_entry(entry, "Daily Spanish");
            history::record_daily_if_new(&date, &entry.id)?;
        }
        Command::Repeat => {
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
            for line in browse::browse_lines(&found) {
                println!("{line}");
            }
        }
        Command::Show { id } => {
            let Some(entry) = entries.iter().find(|entry| entry.id == id) else {
                bail!("no entry with id '{id}'");
            };
            display::print_detail(entry);
        }
        Command::Categories => {
            for line in browse::category_lines(&entries) {
                println!("{line}");
            }
        }
        Command::Tags => {
            for line in browse::tag_lines(&entries) {
                println!("{line}");
            }
        }
        Command::Search { query, filters } => {
            let query = query.join(" ");
            if query.trim().is_empty() {
                bail!("search query must not be empty");
            }
            let found: Vec<content::Entry> = entries
                .iter()
                .filter(|entry| search::matches(entry, &query))
                .cloned()
                .collect();
            let found = filter::apply(&found, &filters)?;
            for line in browse::browse_lines(&found) {
                println!("{line}");
            }
        }
        Command::Save { id } => {
            if !entries.iter().any(|entry| entry.id == id) {
                bail!("no entry with id '{id}'");
            }
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
                let found: Vec<content::Entry> = ids
                    .iter()
                    .filter_map(|id| entries.iter().find(|entry| entry.id == *id))
                    .cloned()
                    .collect();
                for line in browse::browse_lines(&found) {
                    println!("{line}");
                }
            }
        }
        Command::History { limit } => {
            let events = history::recent(limit)?;
            if events.is_empty() {
                println!("No history yet.");
            } else {
                for line in history::lines(&events) {
                    println!("{line}");
                }
            }
        }
    }

    Ok(())
}
