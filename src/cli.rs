use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "dicho", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    /// Show today's entry (the default command)
    Daily,
    /// Show a random entry
    Random,
    /// Show today's entry again
    Repeat,
    /// List all entries in compact form
    Browse,
    /// Show one entry in full
    Show {
        /// Stable entry identifier
        id: String,
    },
    /// List categories with entry counts
    Categories,
    /// List tags with entry counts
    Tags,
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
        for name in ["daily", "random", "repeat", "browse", "categories", "tags"] {
            let cli = Cli::try_parse_from(["dicho", name])
                .unwrap_or_else(|_| panic!("{name} should parse"));
            assert!(cli.command.is_some());
        }
    }

    #[test]
    fn show_requires_an_identifier() {
        assert!(Cli::try_parse_from(["dicho", "show"]).is_err());
        assert!(Cli::try_parse_from(["dicho", "show", "refran-001"]).is_ok());
    }
}
