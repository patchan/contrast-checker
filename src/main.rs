mod colour;
mod wcag;

use clap::Parser;
use colour::Colour;

#[derive(Parser)]
#[command(author, version, about)]
/// Simple tool to calculate the contrast ratio between any two colors.
struct Cli {
    /// Enter a color as a Hex code or a comma separated R,G,B list
    colour1: Colour,

    /// Enter a color as a Hex code or a comma separated R,G,B list
    colour2: Colour,
}

fn main() {
    let cli = Cli::parse();

    let colour1 = cli.colour1;
    let colour2 = cli.colour2;

    wcag::analyze(colour1, colour2);
}

#[cfg(test)]
mod cli_tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        // This checks that the clap configuration is valid
        Cli::command().debug_assert();
    }
}