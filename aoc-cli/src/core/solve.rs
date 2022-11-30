use crate::config::Config;
use aoc::{core::solution_runner::*, helpers::*, solution::*, solutions, util::*};
use arboard::Clipboard;
use futures::executor;
use itertools::Itertools;
use std::{
    collections::HashMap,
    io::{self, Write},
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

fn run_solution_internal(config: &Config, day_type: &SolutionType) -> GenericResult {
    let SolutionInfo { year, day, .. } = day_type.info;
    let year_day = YearDay::new(year, day);
    let print_and_copy = |part: u32, result: &SolveProgress, duration: &Duration| -> () {
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
                let mut clipboard = Clipboard::new().expect("access system clipboard");
                clipboard
                    .set_text(result_text)
                    .expect("write system clipboard");
            }
        }
    };
    println!("\nDay {} - {}", day_type.info.day, day_type.info.title);

    let solver = ThreadSolutionRunner {};
    let stream = solver.run(year_day, Input::Default);
    let pin_stream = Box::into_pin(stream);
    let mut blocking_stream = executor::block_on_stream(pin_stream);
    while let Some(progress) = blocking_stream.next() {
        match &progress {
            SolveProgress::SuccessResult(p) => {
                print_and_copy(p.part.unwrap() as u32, &progress, &p.duration)
            }
            SolveProgress::ErrorResult(p) => {
                print_and_copy(p.part.unwrap() as u32, &progress, &p.duration)
            }
            SolveProgress::Done(p) => println!("Total: {} ms", &p.duration.as_millis()),
            SolveProgress::Error(p) => println!("Error: {}", p),
            SolveProgress::Progress(_) => (),
        }
    }

    Ok(())
}
