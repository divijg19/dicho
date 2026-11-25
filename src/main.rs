use anyhow::Result;
use clap::Parser;

/// A Spanish language companion for your terminal.
#[derive(Debug, Parser)]
#[command(name = "dicho", version, about)]
struct Cli {}

fn main() -> Result<()> {
    let _cli = Cli::parse();
    print_foundation_message();
    Ok(())
}

fn print_foundation_message() {
    println!("dicho: A Spanish language companion for your terminal");
    println!();
    println!("Foundation build (v0.0.2). Daily Spanish content arrives in v0.1.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_without_arguments() {
        assert!(Cli::try_parse_from(["dicho"]).is_ok());
    }

    #[test]
    fn rejects_unknown_arguments() {
        assert!(Cli::try_parse_from(["dicho", "--bogus"]).is_err());
    }
}
