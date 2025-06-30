pub mod analysis;
pub mod cli;
pub mod constants;
pub mod util;

use anyhow::Result;
use clap::Parser;

use crate::analysis::{Rank, RawData, SentenceRanker};
use crate::cli::CLI;

// TODO: [LATER] add database support, either Diesel or rusqlite [SQLite-based].

fn main() -> Result<()> {
    let cli = CLI::parse();
    if !cli.file.exists() {
        println!("ERROR: file does not exist or the file path provided is incorrect.");
        return Ok(());
    }
    let data = RawData::from_file(cli.file.to_str().unwrap())?;
    let ranks = SentenceRanker::new(data);
    for Rank { sentence, score } in ranks.rankings() {
        println!("- [{}]: {};", sentence, score);
    }

    Ok(())
}
