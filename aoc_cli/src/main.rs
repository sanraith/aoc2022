use aoc::{solution::*, solutions::*};
use aoc_cli::{args::*, scaffold, timing};
use clap::Parser;
use clearscreen;
use std::io::{self, Write};

fn main() {
    _ = clearscreen::clear();
    println!("--- Advent of Code 2022 CLI by sanraith ---");

    let args = Args::parse();
    match args.mode {
        Some(Command::Scaffold { year, days }) => scaffold(year, days),
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

fn scaffold(year: Option<i32>, days: Vec<u32>) {
    let year = match year {
        Some(year) => year,
        None => timing::latest_aoc_date().year,
    };

    match days.len() {
        1.. => days
            .into_iter()
            .for_each(|day| scaffold::scaffold_day(year, day)),
        _ => scaffold::scaffold_day(year, timing::latest_aoc_date().day),
    };
}
