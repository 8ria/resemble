//! Command-line tool for computing similarity between two Rust files.

use anyhow::{Context, Result};
use resemble::{parse_and_count, similarity::similarity_from_counts};
use std::{env, path::Path};

/// Prints usage and exits the program.
fn print_usage_and_exit() -> ! {
    eprintln!("Usage: resemble <file_a.rs> <file_b.rs>");
    eprintln!("Ensure that both Rust source files exist before comparing.");
    std::process::exit(2);
}

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let a = args.next().unwrap_or_else(|| print_usage_and_exit());
    let b = args.next().unwrap_or_else(|| print_usage_and_exit());

    if !Path::new(&a).exists() || !Path::new(&b).exists() {
        eprintln!("Error: One or both files do not exist.");
        eprintln!("Please create `file_a.rs` and `file_b.rs` or provide valid paths.");
        std::process::exit(1);
    }

    let counts_a = parse_and_count(&a).with_context(|| format!("Failed to parse '{}'", &a))?;
    let counts_b = parse_and_count(&b).with_context(|| format!("Failed to parse '{}'", &b))?;

    let sim = similarity_from_counts(counts_a, counts_b);

    println!("Cosine similarity = {:.6}", sim);
    Ok(())
}
