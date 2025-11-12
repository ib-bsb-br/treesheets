mod display;

use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde_json::Value;
use treesheets_core::{load_sheet, save_sheet, validate_sheet_json, Sheet};

use crate::display::print_sheet;

#[derive(Parser)]
#[command(name = "TreeSheets RS", author, version, about = "Rust-based TreeSheets prototype", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Loads a sheet from JSON and prints a textual view to stdout.
    Print {
        /// Path to a JSON file produced by TreeSheets RS.
        input: PathBuf,
    },
    /// Generates the bundled sample sheet and writes it to disk.
    Sample {
        /// Output path that will receive the generated JSON representation.
        output: PathBuf,
        /// Overwrite the destination file when it already exists.
        #[arg(short, long)]
        force: bool,
    },
    /// Validates that the JSON file follows the expected schema.
    Validate {
        /// Path to a JSON file to validate.
        input: PathBuf,
    },
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err:?}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Print { input } => {
            let sheet =
                load_sheet(&input).with_context(|| format!("loading sheet from {input:?}"))?;
            print_sheet(&sheet);
        }
        Command::Sample { output, force } => {
            if !force && output.exists() {
                anyhow::bail!("{output:?} already exists; pass --force to overwrite");
            }
            let sheet = Sheet::sample();
            save_sheet(&output, &sheet)
                .with_context(|| format!("writing sample sheet to {output:?}"))?;
        }
        Command::Validate { input } => {
            let text = fs::read_to_string(&input)
                .with_context(|| format!("reading {input:?} for validation"))?;
            let value: Value = serde_json::from_str(&text)
                .with_context(|| format!("parsing {input:?} as JSON"))?;
            if validate_sheet_json(&value) {
                println!("{input:?} is a valid TreeSheets RS sheet");
            } else {
                anyhow::bail!("{input:?} does not match the TreeSheets RS schema");
            }
        }
    }

    Ok(())
}
