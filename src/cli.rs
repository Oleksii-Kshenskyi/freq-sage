use std::path::PathBuf;

use clap::{ArgAction, Parser, Subcommand, ValueEnum};

pub const FREQSAGE_ABOUT_SHORT: &str = "Frequency analysis of text for language learning!";
pub const FREQSAGE_ABOUT: &str = "FreQ Sage is an application for frequency analysis of text, mostly for the purpose of language learning and, specifically, sentence mining.";

#[derive(Parser, Debug)]
#[command(name = "FreQ Sage")]
#[command(
    version,
    about = FREQSAGE_ABOUT_SHORT,
    long_about = FREQSAGE_ABOUT,
)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,
    // TODO: develop a system for processing texts based on a number of pre-existing presets for specific text sources: such as Gutenberg books, Wikipedia articles, etc.
    // TODO: Introduce and develop a new argument that controls starting from a specific rank (only start from the 10th sentence, for example)
    // TODO[[1]]: Introduce and develop a new argument that restricts the total number of sentences shown in the output. Also by default, only show the first X sentences starting from the rank specified in the argument (that is to say, introduce a default value to the argument developed in [[1]] and make it a setting in constants.rs/.env).
    // TODO[[2]] [MAIN FEATURE]: Related to [[1]]: develop a feature (called `random` or `topN`, or `batch`) that shows N random sentences. There are two sub-modes for this:
    // 1. Have a feature to restrict sentences to showing only random sentences with easiness of up-to-X (not-harder-than-X-easiness). This is done so that we can limit the application from showing sentences that are too complex for the current level.
    // 2. Have a feature of only showing new sentences on subsequent runs of the command. Even though the sentences are random, only show those that haven't been shown on previous runs.
    // 3. Have a feature to regen only specific sentences out of the N generated ones. For example, if 20 sentences have been generated and you only don't like sentences 3, 7 and 15, you need to be able to specify that you only want those to be regenerated, and that's it. Other 17 should remain as is.
    // 4. Have a feature to print those final regenerated and now-suitable N sentences to a new file.
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Train FreQSage by giving it a text to analyze.")]
    Train {
        #[arg(help = "The text file to analyze [REQUIRED].")]
        file: PathBuf,
    },
    #[command(about = "Show top word frequencies or sentence rankings.")]
    Show {
        #[arg(value_enum, help = "what to show: sentences or rankings.")]
        what: ShowType,
        #[arg(
            short = 'l',
            long = "limit",
            help = "If specified, limits the queries of top rankings/frequencies to this specific amount. If left unspecified, the value of DEFAULT_TOP_N_LIMIT env variable is used, usually 50. Can be changed in .env (see .env.template)."
        )]
        limit: Option<u32>,
        #[arg(
            conflicts_with = "limit",
            short = 'n',
            long = "no-limit",
            action = ArgAction::SetTrue,
            help = "The opposite (and obviously conflicts with) of `-l/--limit`. If this is specified, no limit is applied to the top queries at all - no .env variable, no flag, no default, nothing. This will print as many entries as there are in the database. WARNING: may print A LOT if you've been using your DB for a while."
        )]
        no_limit: bool,
    },
}

#[derive(Debug, ValueEnum, Clone)]
pub enum ShowType {
    #[value(help = "Show top N word frequencies.")]
    Frequencies,
    #[value(help = "Show top N sentence rankings.")]
    Rankings,
}
