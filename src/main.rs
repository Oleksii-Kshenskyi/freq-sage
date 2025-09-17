pub mod analysis;
pub mod cli;
pub mod constants;
pub mod database;
pub mod schema;
pub mod util;

use anyhow::{Context, Result};
use clap::Parser;
use dotenvy;

use crate::analysis::RawData;
use crate::cli::CLI;
use crate::constants::DEFAULT_LANGUAGE;
use crate::database::Database;

// TODO: [AFTER DB] Apart from the ability to train the database, Sage must also have the ability to dry-run and just show the rankings of sentences in this specific text, without adding the info to the DB. (training and dry-running should potentially be two different subcommands?)
// TODO: [POTENTIALLY] implement support of several file formats so (for example) processing PDF books becomes possible.
// TODO: Implement the ability to clean up the database based on a regex filter (remove all sentences that match the filter from the database).

fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let mut db = Database::new(&std::env::var("DATABASE_URL")?)?;
    let dlang = std::env::var("DEFAULT_LANGUAGE").unwrap_or(DEFAULT_LANGUAGE.to_owned());

    let cli = CLI::parse();
    match cli.command {
        // TODO: It should be possible to explicitly set the language of the file you're training with in the CLI parameters of train mode.
        cli::Commands::Train { file } => {
            let data = RawData::from_file(file.to_str().unwrap(), dlang)?;
            db.insert_freqs(&data)
                .context("main(): while trying to insert new freqs into the database.")?;
            let sizes = data.data_sizes();
            db.insert_rankings(&data)
                .context("main(): while trying to insert rankings into the database.")?;
            println!(
                "SUCCESS: processed file `{}` containing {} frequencies and {} sentences.",
                file.display(),
                sizes.0,
                sizes.1
            );
        }
        cli::Commands::Show { what } => match what {
            cli::ShowType::Frequencies => {
                for (index, freq) in db.top_freqs(None)?.iter().enumerate() {
                    println!("{}. `{}`: {}", index + 1, freq.word, freq.frequency)
                }
            }
            cli::ShowType::Rankings => {
                for (index, rank) in db.top_rankings(None)?.iter().enumerate() {
                    println!(
                        "{}. `{}` [{}]: {}",
                        index + 1,
                        rank.sentence,
                        rank.lang,
                        rank.ranking
                    )
                }
            }
        },
    }

    Ok(())
}
