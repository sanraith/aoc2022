use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub mode: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Scaffold solution and test files
    Scaffold {
        /// Specifies the target year. Defaults to the latest available AOC year.
        #[arg(short, long)]
        year: Option<i32>,
        /// List of days to scaffold. Defaults to [the latest available AOC day].
        days: Vec<u32>,
    },
    /// Solve puzzles
    Solve {
        /// List of days to solve. Defaults to [all implemented days].
        days: Vec<u8>,
    },
}
