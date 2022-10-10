use aoc2022::{
    cli::*,
    scaffold::{self, Target},
    solution::*,
    solutions::Day01,
};
use clap::Parser;
use clearscreen;
use std::io::{self, Write};

fn run_solution() {
    let context = Context {
        raw_input: "123\n456\n789".to_owned(),
        on_progress: |p| {
            print!("\r{:.2}%  ", p * 100.0);
            io::stdout().flush().unwrap();
        },
    };
    let mut solution = Day01::new();
    let result = solution.part1(&context);
    println!();

    println!("{}", result.unwrap_or_else(|x| format!("Error: {}", x)));
}

fn main() {
    clearscreen::clear().ok();
    println!("--- Advent of Code 2022 CLI by sanraith ---");

    let cli = Args::parse();
    match cli.mode {
        Some(Command::Scaffold { days }) => {
            let days = match days.len() {
                0 => vec![0], // TODO next available day
                _ => days,
            };
            scaffold::scaffold(Target::Days(&days));
        }
        Some(Command::Solve { days }) => {
            let days_str = days
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            println!("Solving days: {}", days_str);
        }
        None => {
            println!("Default behavior");
            run_solution();
        }
    }
}
