use std::path::PathBuf;

use clap::Parser;

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
    #[arg(help = "The text file to analyze. [REQUIRED]")]
    pub file: PathBuf,
    // TODO: develop a system for processing texts based on a number of pre-existing presets for specific text sources: such as Gutenberg books, Wikipedia articles, etc.
}
