use crate::config::Config;
use aoc::{core::solution_runner::*, helpers::*, solution::*, solutions, util::*};
use arboard::Clipboard;
use itertools::Itertools;
use std::{
    collections::HashMap,
    io::{self, Write},
    thread,
    time::{Duration, SystemTime},
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
    let mut result_text = match result {
        SolveProgress::SuccessResult(r) => r.value.clone(),
        SolveProgress::ErrorResult(r) => format!("Error - {}", &r.value),
        _ => panic!("Not supported result!"),
    };

    if result_text.contains("\n") {
        result_text = format!("\n{}", &result_text);
    }

    println!(
        "Part {} ({}): {}",
        part,
        fmt_duration(duration),
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
        let mut _dbg_loop_count = 0;
        let mut _dbg_lock_duration = Duration::default();
        let mut _dbg_sleep_duration = Duration::default();

        loop {
            _dbg_loop_count += 1;
            let before_lock = SystemTime::now();
            // If we lock in 'while let' or 'match' instead, we would lock while sleeping
            let next_items = stream.lock().unwrap().next_items();
            _dbg_lock_duration += before_lock.elapsed().unwrap_or_default();

            let items = match next_items {
                Some(items) if items.len() > 0 => items,
                Some(_) => {
                    let before_sleep = SystemTime::now();
                    thread::sleep(Duration::from_millis(10));
                    _dbg_sleep_duration += before_sleep.elapsed().unwrap_or_default();
                    continue;
                }
                None => break,
            };

            for progress in items {
                match &progress {
                    SolveProgress::SuccessResult(p) => {
                        print_and_copy(p.part.unwrap() as u32, &progress, &p.duration, &config)
                    }
                    SolveProgress::ErrorResult(p) => {
                        print_and_copy(p.part.unwrap() as u32, &progress, &p.duration, &config)
                    }
                    SolveProgress::Done(p) => println!("Total: {}", fmt_duration(&p.duration)),
                    SolveProgress::Error(p) => println!("Error: {}", p),
                    SolveProgress::Progress(p) => println!(
                        "Progress ({}): {:.2}%",
                        fmt_duration(&p.duration),
                        p.value * 100.0
                    ),
                }
            }
        }

        // println!(
        //     "loops: {}, lock: {:?}, sleep: {:?}",
        //     _dbg_loop_count, _dbg_lock_duration, _dbg_sleep_duration
        // );
    });
    t.join().unwrap();

    Ok(())
}
