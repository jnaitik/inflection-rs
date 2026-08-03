//! CLI tool for the `inflection` library.
//!
//! Provides subcommands for all inflection functions, reading input from
//! command-line arguments or stdin.

use clap::{Parser, Subcommand};
use thiserror::Error;

/// inflection — Transform English words between various forms.
///
/// A CLI tool for singular/plural conversion, CamelCase/snake_case transformation,
/// URL slug generation, and more. Rust port of the Python `inflection` library.
#[derive(Parser)]
#[command(name = "inflection", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert to CamelCase (UpperCamelCase by default)
    Camelize {
        /// Input string
        input: String,
        /// Use lowerCamelCase instead of UpperCamelCase
        #[arg(short, long)]
        lower: bool,
    },
    /// Replace underscores with dashes
    Dasherize {
        /// Input string
        input: String,
    },
    /// Convert to human-readable form
    Humanize {
        /// Input string
        input: String,
    },
    /// Get the ordinal suffix for a number (st, nd, rd, th)
    Ordinal {
        /// The number
        number: i64,
    },
    /// Convert a number to its ordinal form (1st, 2nd, 3rd, etc.)
    Ordinalize {
        /// The number
        number: i64,
    },
    /// Create a URL-friendly slug
    Parameterize {
        /// Input string
        input: String,
        /// Separator character (default: "-")
        #[arg(short, long, default_value = "-")]
        separator: String,
    },
    /// Convert to plural form
    Pluralize {
        /// Input word
        input: String,
    },
    /// Convert to singular form
    Singularize {
        /// Input word
        input: String,
    },
    /// Create a table name from a model name
    Tableize {
        /// Input string
        input: String,
    },
    /// Convert to a title
    Titleize {
        /// Input string
        input: String,
    },
    /// Transliterate Unicode to ASCII
    Transliterate {
        /// Input string
        input: String,
    },
    /// Convert CamelCase to snake_case
    Underscore {
        /// Input string
        input: String,
    },
}

#[derive(Error, Debug)]
enum CliError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

fn main() -> Result<(), CliError> {
    let cli = Cli::parse();

    let output = match cli.command {
        Commands::Camelize { input, lower } => inflection::camelize(&input, !lower),
        Commands::Dasherize { input } => inflection::dasherize(&input),
        Commands::Humanize { input } => inflection::humanize(&input),
        Commands::Ordinal { number } => inflection::ordinal(number).to_string(),
        Commands::Ordinalize { number } => inflection::ordinalize(number),
        Commands::Parameterize { input, separator } => inflection::parameterize(&input, &separator),
        Commands::Pluralize { input } => inflection::pluralize(&input),
        Commands::Singularize { input } => inflection::singularize(&input),
        Commands::Tableize { input } => inflection::tableize(&input),
        Commands::Titleize { input } => inflection::titleize(&input),
        Commands::Transliterate { input } => inflection::transliterate(&input).to_string(),
        Commands::Underscore { input } => inflection::underscore(&input),
    };

    println!("{output}");
    Ok(())
}
