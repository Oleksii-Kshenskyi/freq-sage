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
    // TODO: Introduce and develop a new argument that controls starting from a specific rank (only start from the 10th sentence, for example) [1]
    // TODO: Introduce and develop a new argument that restricts the total number of sentences shown in the output. Also by default, only show the first X sentences starting from the (1) rank (that is to say, introduce a default value to this (2) argument). [2]
}
