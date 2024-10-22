pub mod board;
pub mod dictionary;

use board::Board;
use clap::{Args, Parser, Subcommand};

/// A 7x7.game solver
#[derive(Debug, Parser)]
#[clap(name = "seven-solver")]
#[clap(about="A 7x7.game solver", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

static TODAY: &'static str = "<TODAY>";

#[derive(Debug, Subcommand)]
enum Commands {
    /// Fetches boards
    #[clap(arg_required_else_help = false)]
    Fetch {
        /// The date to fetch
        #[arg(default_value=TODAY)]
        when: String,
    },
    /// Solves a given board
    Solve(SolveArgs),
}
/// Figure out the perfect solution for a game of 7x7
#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct SolveArgs {
    /// Where them words at?
    #[arg(long, default_value = "resources/7x7-word-list-sevens-only.txt")]
    dict_path: String,

    #[arg(short = 'w', long, default_value_t = 7)]
    min_word_length: usize,

    #[arg(default_value = TODAY)]
    date: String,

    /// Quiet - don't show any output: overrides --memory-debug
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
}

fn fetch(when: &str) -> Board {
    if when == TODAY {
        println!("fetching today's board");
        board::Board::get_todays_board()
    } else {
        println!("Fetching {}", when);
        board::Board::get_board_for(&when)
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Fetch { when } => {
            _ = fetch(&when);
        }
        Commands::Solve(s) => {
            let b = fetch(&s.date);
            let dict = dictionary::Dictionary::new(&s.dict_path, s.min_word_length, s.quiet);
            println!("{:?}", b);

            let words = b.find_words(&dict);
            println!("Found words? {:?}", words);
        }
    }
}
