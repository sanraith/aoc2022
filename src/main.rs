use std::io::{self, Write};

use aoc2022::{api::solution::*, solutions::*};

pub fn run_solution() {
    let context = Context {
        input: "asd",
        progress: |p| {
            print!("\r{:.2}%  ", p * 100.0);
            io::stdout().flush().unwrap();
        },
    };
    let solution = Day01::new();
    let result = solution.part1(&context);
    println!();

    println!("{}", result.unwrap_or_else(|x| format!("Error: {}", x)));
}

fn main() {
    println!("Hello World!");
    run_solution();
}
