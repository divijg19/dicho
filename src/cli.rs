use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "dicho", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Clone, Copy, Subcommand)]
pub enum Command {
    /// Show today's entry (the default command)
    Daily,
    /// Show a random entry
    Random,
    /// Show today's entry again
    Repeat,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_command_uses_daily() {
        let cli = Cli::try_parse_from(["dicho"]).expect("root command should parse");
        assert!(cli.command.is_none());
    }

    #[test]
    fn subcommands_parse() {
        for name in ["daily", "random", "repeat"] {
            let cli = Cli::try_parse_from(["dicho", name])
                .unwrap_or_else(|_| panic!("{name} should parse"));
            assert!(cli.command.is_some());
        }
    }
}
