pub mod board;
pub mod dictionary;

use clap::Parser;
//use clio::*;

/// Figure out the perfect solution for a game of 7x7
#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Args {
    /// Where them words at?
    #[arg(long, default_value = "resources/7x7-word-list.txt")]
    dict_path: String,

    #[arg(short = 'w', long, default_value_t = 7)]
    min_word_length: usize,

    /// Quiet - don't show any output: overrides --memory-debug
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
}

fn main() {
    let args = Args::parse();

    let b = board::Board::get_todays_board();
    println!("{:?}", b);

    let dict = dictionary::Dictionary::new(&args);
    let words = b.find_words(&dict);
    println!("Found words? {:?}", words);
}
