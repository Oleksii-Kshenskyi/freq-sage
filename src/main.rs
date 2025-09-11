pub mod analysis;
pub mod cli;
pub mod constants;
pub mod database;
pub mod schema;
pub mod util;

use anyhow::{Context, Result};
use clap::Parser;
use diesel::prelude::*;
use dotenvy;

use crate::analysis::{Rank, RawData, SentenceRanker};
use crate::cli::CLI;
use crate::constants::DEFAULT_LANGUAGE;
use crate::database::Database;

// TODO: [LATER] add database support, either Diesel or rusqlite [SQLite-based].
// TODO: [AFTER DB] Develop a mechanism for FreQ Sage to train the database on new texts
// TODO: [AFTER DB] Apart from the ability to train the database, Sage must also have the ability to dry-run and just show the rankings of sentences in this specific text, without adding the info to the DB. (training and dry-running should potentially be two different subcommands?)
//       [POTENTIALLY] implement support of several file formats so (for example) processing
//                     PDF books becomes possible.
// TODO: Implement separating "databases" based on languages, separate database for each language.
// TODO: In the future, if/when there's a settings file, implement a "default language" setting.
// TODO: Implement the ability to clean up the database based on a regex filter (remove all sentences that match the filter from the database).

// TODO: [!!!] Fix all the warnings first!
fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let mut db = Database::new(&std::env::var("DATABASE_URL")?)?;
    let dlang = std::env::var("DEFAULT_LANGUAGE").unwrap_or(DEFAULT_LANGUAGE.to_owned());

    let cli = CLI::parse();
    match cli.command {
        // FIXME: [!!!] training kind of works, but it re-adds the same sentences to the database, duplicating them instead of changing for uniqueness?
        cli::Commands::Train { file } => {
            let data = RawData::from_file(file.to_str().unwrap())?;
            db.insert_freqs(&data.freqs);
            db.insert_rankings(data);
        }
        cli::Commands::Show { what } => match what {
            cli::ShowType::Frequencies => {
                for (index, freq) in db.top_freqs(None)?.iter().enumerate() {
                    println!("{}. `{}`: {}", index + 1, freq.word, freq.frequency)
                }
            }
            cli::ShowType::Rankings => {
                for (index, rank) in db.top_rankings(None)?.iter().enumerate() {
                    println!("{}. `{}`: {}", index + 1, rank.sentence, rank.ranking)
                }
            }
        },
    }

    Ok(())
}
