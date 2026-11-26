mod cli;
mod content;
mod daily;
mod display;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let entries = content::load()?;
    let date = daily::today()?;
    let command = cli.command.unwrap_or(Command::Daily);

    let (entry, heading) = match command {
        Command::Daily => (daily::daily(&entries, &date), "Daily Spanish"),
        Command::Repeat => (daily::daily(&entries, &date), "Daily Spanish"),
        Command::Random => (daily::random(&entries, &date), "Random Spanish"),
    };

    display::print_entry(entry, heading);
    Ok(())
}
