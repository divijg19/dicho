use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "dicho", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Clone, Default, Args)]
pub struct Filters {
    /// Only entries in this category
    #[arg(long, value_name = "CATEGORY")]
    pub category: Option<String>,
    /// Only entries with this tag
    #[arg(long, value_name = "TAG")]
    pub tag: Option<String>,
    /// Only entries at or above this difficulty
    #[arg(long, value_name = "VALUE")]
    pub min_difficulty: Option<f32>,
    /// Only entries at or below this difficulty
    #[arg(long, value_name = "VALUE")]
    pub max_difficulty: Option<f32>,
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
    Browse {
        #[command(flatten)]
        filters: Filters,
    },
    /// Show one entry in full
    Show {
        /// Stable entry identifier
        id: String,
    },
    /// List categories with entry counts
    Categories,
    /// List tags with entry counts
    Tags,
    /// Search entries across text, translation, tags, source, and vocabulary
    Search {
        /// Terms to search for
        #[arg(required = true)]
        query: Vec<String>,
        #[command(flatten)]
        filters: Filters,
    },
    /// Save an entry for later
    Save {
        /// Stable entry identifier
        id: String,
    },
    /// Remove an entry from saved items
    Unsave {
        /// Stable entry identifier
        id: String,
    },
    /// List saved entries
    Saved,
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
        for name in [
            "daily",
            "random",
            "repeat",
            "browse",
            "categories",
            "tags",
            "saved",
        ] {
            let cli = Cli::try_parse_from(["dicho", name])
                .unwrap_or_else(|_| panic!("{name} should parse"));
            assert!(cli.command.is_some());
        }
    }

    #[test]
    fn search_requires_a_query() {
        assert!(Cli::try_parse_from(["dicho", "search"]).is_err());
        assert!(Cli::try_parse_from(["dicho", "search", "un", "clavo"]).is_ok());
    }

    #[test]
    fn show_requires_an_identifier() {
        assert!(Cli::try_parse_from(["dicho", "show"]).is_err());
        assert!(Cli::try_parse_from(["dicho", "show", "refran-001"]).is_ok());
    }

    #[test]
    fn browse_filters_parse() {
        let cli = Cli::try_parse_from([
            "dicho",
            "browse",
            "--category",
            "proverb",
            "--tag",
            "time",
            "--min-difficulty",
            "0.2",
            "--max-difficulty",
            "0.8",
        ])
        .expect("filters should parse");
        let Command::Browse { filters } = cli.command.expect("command") else {
            panic!("expected browse");
        };
        assert_eq!(filters.category.as_deref(), Some("proverb"));
        assert_eq!(filters.tag.as_deref(), Some("time"));
        assert_eq!(filters.min_difficulty, Some(0.2));
        assert_eq!(filters.max_difficulty, Some(0.8));
    }
}
