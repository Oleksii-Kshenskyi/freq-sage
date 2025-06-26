pub mod cli;

use clap::Parser;
use cli::CLI;

// TODO: first, write a simple CLI that counts frequencies of words in a file and prints the report on screen.

fn main() {
    let cli = CLI::parse();
    if cli.kekw {
        println!("THIS IS A HELPFUL HELP MESSAGE!!");
    } else {
        println!("No help PepeHands T_T");
    }
}
