extern crate assignment;

use std::io::{self};
use assignment::*;
use assignment::utils::*;

type TrialPair = (&'static str, fn () ->());

const runs: [TrialPair; 3] = [
    ("Gamblers Ruin", run_gamblers), 
    ("Absorption", run_absorption),
    ("Polya Urn", run_polya)
];

fn main() -> io::Result<()> {
    let enumerated = runs.iter()
        .enumerate()
        .map(|(i, (p, _))| format!("{}. {}", i + 1, p))
        .collect::<Vec<String>>()
        .join("\n");

    let prompt = format!("Choose an experiment:\n{}\n", enumerated);
    let chosen = read_until_checked(prompt.as_str(), check_in_range(1, runs.len() + 1)) - 1;

    runs[chosen].1();
    // run_polya();

    println!("Press enter to continue...");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(())
}
