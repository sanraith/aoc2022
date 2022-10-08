use aoc2022::{api::solution::*, solutions::*};

pub fn run_solution() {
    let context = Context { input: "asd" };
    let solution = Day01::new();
    let result = solution.part1(&context);

    println!("{}", result.unwrap_or_else(|x| format!("Error: {}", x)));
}

fn main() {
    println!("Hello World!");
    run_solution();
}
