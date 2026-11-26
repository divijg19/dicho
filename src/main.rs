mod browse;
mod cli;
mod content;
mod daily;
mod display;
mod search;

use anyhow::{Result, bail};
use clap::Parser;
use cli::{Cli, Command};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let entries = content::load()?;
    let command = cli.command.unwrap_or(Command::Daily);

    match command {
        Command::Daily => {
            let entry = daily::daily(&entries, &daily::today()?);
            display::print_entry(entry, "Daily Spanish");
        }
        Command::Repeat => {
            let entry = daily::daily(&entries, &daily::today()?);
            display::print_entry(entry, "Daily Spanish");
        }
        Command::Random => {
            let entry = daily::random(&entries, &daily::today()?);
            display::print_entry(entry, "Random Spanish");
        }
        Command::Browse => {
            for line in browse::browse_lines(&entries) {
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
        Command::Search { query } => {
            let query = query.join(" ");
            if query.trim().is_empty() {
                bail!("search query must not be empty");
            }
            let found: Vec<content::Entry> = entries
                .iter()
                .filter(|entry| search::matches(entry, &query))
                .cloned()
                .collect();
            for line in browse::browse_lines(&found) {
                println!("{line}");
            }
        }
    }

    Ok(())
}
