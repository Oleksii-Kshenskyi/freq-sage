pub mod analysis;
pub mod cli;
pub mod constants;
pub mod util;

use anyhow::Result;
use clap::Parser;

use crate::analysis::RawData;
use crate::cli::CLI;

// TODO: first, write a simple CLI that counts frequencies of words in a file and prints the report on screen.
// TODO: Then, add database support, either Diesel or rusqlite [SQLite-based].

fn main() -> Result<()> {
    let cli = CLI::parse();
    if !cli.file.exists() {
        println!("ERROR: file does not exist or the file path provided is incorrect.");
        return Ok(());
    }
    let data = RawData::from_file(cli.file.to_str().unwrap())?;
    let mut pairs: Vec<(&String, &u64)> = data.freqs.iter().collect();
    pairs.sort_by(|a, b| a.1.cmp(b.1));
    for (word, freq) in pairs {
        println!("- [{}]: {};", word, freq);
    }

    Ok(())
}
