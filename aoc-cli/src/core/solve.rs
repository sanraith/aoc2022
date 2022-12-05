use crate::config::Config;
use aoc::{core::solution_runner::*, helpers::*, solution::*, solutions, util::*};
use arboard::Clipboard;
use itertools::Itertools;
use std::{
    collections::HashMap,
    io::{self, Write},
    thread,
    time::Duration,
};

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

fn print_and_copy(part: u32, result: &SolveProgress, duration: &Duration, config: &Config) {
    let result_text = match result {
        SolveProgress::SuccessResult(r) => r.value.clone(),
        SolveProgress::ErrorResult(r) => format!("Error - {}", &r.value),
        _ => panic!("Not supported result!"),
    };

    println!(
        "Part {} ({} ms): {}",
        part,
        duration.as_millis(),
        &result_text
    );
    if config.copy_result_to_clipboard {
        if let SolveProgress::SuccessResult(_) = &result {
            match Clipboard::new() {
                Ok(mut clipboard) => {
                    if let Err(err) = clipboard.set_text(result_text) {
                        println!("Warning: could not copy output to clipboard! {}", err);
                    }
                }
                Err(err) => println!("Warning: could access clipboard! {}", err),
            }
        }
    }
}

fn run_solution_internal(config: &Config, day_type: &SolutionType) -> GenericResult {
    let SolutionInfo { year, day, .. } = day_type.info;
    let year_day = YearDay::new(year, day);
    println!("\nDay {} - {}", day_type.info.day, day_type.info.title);

    let solver = ThreadSolutionRunner {};
    let stream = solver.run(year_day, Input::Default);

    let config = config.clone();
    let t = thread::spawn(move || {
        while let Some(items) = stream.lock().unwrap().next_items() {
            if items.len() == 0 {
                thread::sleep(Duration::from_millis(20));
                continue;
            }

            for progress in items {
                match &progress {
                    SolveProgress::SuccessResult(p) => {
                        print_and_copy(p.part.unwrap() as u32, &progress, &p.duration, &config)
                    }
                    SolveProgress::ErrorResult(p) => {
                        print_and_copy(p.part.unwrap() as u32, &progress, &p.duration, &config)
                    }
                    // SolveProgress::Done(p) => println!("Total: {} ms", &p.duration.as_millis()),
                    SolveProgress::Done(_) => (),
                    SolveProgress::Error(p) => println!("Error: {}", p),
                    SolveProgress::Progress(p) => println!("Progress: {:.2}%", p.value * 100.0),
                }
            }
        }
    });
    t.join().unwrap();

    Ok(())
}
