use aoc::{helpers::*, inputs, solution::*, solutions, util::*};
use arboard::Clipboard;
use itertools::Itertools;
use std::{
    collections::HashMap,
    io::{self, Write},
};

use crate::config::Config;

pub fn run_solutions(config: &Config) -> GenericResult {
    let solutions = solutions::create_map();
    let solutions_by_year: HashMap<i32, Vec<&SolutionType>> =
        solutions.iter().fold(HashMap::new(), |mut map, (k, v)| {
            map.entry(k.year).or_insert_with(|| Vec::new()).extend(v);
            map
        });
    let sorted_solutions = solutions_by_year
        .into_iter()
        .sorted_by(|(a, _), (b, _)| a.cmp(b))
        .map(|(y, days)| (y, days.into_sorted_by(|a, b| a.info.day.cmp(&b.info.day))));

    for (year, solutions) in sorted_solutions {
        println!("\n--- Year {} ---", year);
        for day in solutions {
            run_solution_internal(config, day)?
        }
    }

    Ok(())
}

pub fn run_solution(config: &Config, year: i32, day: u32) -> GenericResult {
    let solutions = solutions::create_map();
    let day_type = solutions
        .get(&YearDay { year, day })
        .and_then(|x| x.first())
        .ok_or(MsgError("solution cannot be found"))?;
    run_solution_internal(config, day_type)
}

struct HandleProgress;
impl ProgressHandler for HandleProgress {
    fn on_progress(&mut self, value: f32) {
        print!("\r{:.2}%  ", value * 100.0);
        io::stdout().flush().unwrap();
    }
}

fn run_solution_internal(config: &Config, day_type: &SolutionType) -> GenericResult {
    let SolutionInfo { year, day, .. } = day_type.info;
    let year_day = YearDay::new(year, day);
    let raw_input = inputs::get(&year_day)
        .ok_or(MsgError("input should be available"))?
        .to_owned();
    let ctx = Context {
        raw_input,
        progress_handler: Box::new(HandleProgress),
    };
    let print_and_copy = |part: u32, result: &SolutionResult| -> () {
        println!(
            "Part {}: {}",
            part,
            result
                .as_ref()
                .map(|x| x.to_owned())
                .unwrap_or_else(|x| format!("Error - {}", x))
        );
        if config.copy_result_to_clipboard {
            if let Ok(result) = &result {
                let mut clipboard = Clipboard::new().expect("access system clipboard");
                clipboard.set_text(result).expect("write system clipboard");
            }
        }
    };
    println!("\nDay {} - {}", day_type.info.day, day_type.info.title);
    let mut day = day_type.create_new();
    _ = day
        .init(&ctx)
        .and_then(|_| day.part1(&ctx))
        .tap(|result| print_and_copy(1, result))
        .and_then(|_| day.part2(&ctx))
        .tap(|result| print_and_copy(2, result));
    Ok(())
}
