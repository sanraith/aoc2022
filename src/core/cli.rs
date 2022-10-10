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
        /// List of days to scaffold. Scaffold next day if left empty.
        days: Vec<u8>,
    },
    /// Solve puzzles
    Solve {
        /// List of days to solve. Solves all days if empty.
        days: Vec<u8>,
    },
}
