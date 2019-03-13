extern crate assignment;

use assignment::utils::*;
use assignment::*;
use std::io;

type TrialPair = (&'static str, fn() -> ());

const RUNS: [TrialPair; 3] = [
    ("Gamblers Ruin", run_gamblers),
    ("Absorption", run_absorption),
    ("Polya Urn", run_polya),
];

fn main() -> io::Result<()> {
    let enumerated = RUNS
        .iter()
        .enumerate()
        .map(|(i, (p, _))| format!("{}. {}", i + 1, p))
        .collect::<Vec<String>>()
        .join("\n");

    let prompt = format!("Choose an experiment:\n{}\n", enumerated);
    let chosen = read_until_checked(prompt.as_str(), check_in_range(1, RUNS.len() + 1)) - 1;

    RUNS[chosen].1();

    println!("Press enter to continue...");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(())
}
