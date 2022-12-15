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

pub fn run_all_solutions(config: &Config) -> GenericResult<Duration> {
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

    let mut total_duration = Duration::default();
    for (year, solutions) in sorted_solutions {
        println!("\n--- Year {} ---", year);
        for day in solutions {
            total_duration += run_solution_internal(config, day)?;
        }
    }

    Ok(total_duration)
}

pub fn run_solution(config: &Config, year: i32, day: u32) -> GenericResult<Duration> {
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

/// Prints over the current line with \r
fn print(content: &str, prev_line_length: usize) -> usize {
    let spaces = String::from_iter(
        (0..(prev_line_length as i32 - content.len() as i32).max(0)).map(|_| ' '),
    );
    print!("\r{}", format!("{}{}", content, spaces));
    io::stdout().flush().unwrap();
    content.len()
}

fn print_and_copy(
    part: u32,
    result: &SolveProgress,
    duration: &Duration,
    config: &Config,
    prev_line_length: usize,
) {
    let mut result_text = match result {
        SolveProgress::SuccessResult(r) => r.value.clone(),
        SolveProgress::ErrorResult(r) => format!("Error - {}", &r.value),
        _ => panic!("Not supported result!"),
    };

    // Display multiline results in an empty line
    if result_text.contains("\n") {
        result_text = format!("\n{}", &result_text);
    }
    let content = format!(
        "Part {} ({}): {}",
        part,
        fmt_duration(duration),
        &result_text
    );
    print(&content, prev_line_length);
    println!("");

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
    };
}

fn run_solution_internal(config: &Config, day_type: &SolutionType) -> GenericResult<Duration> {
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

        let mut prev_line_length = print("Part 1...", 0);
        let mut solution_duration = Duration::default();
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
                        print_and_copy(
                            p.part.unwrap() as u32,
                            &progress,
                            &p.duration,
                            &config,
                            prev_line_length,
                        );

                        if p.part == Some(1) {
                            prev_line_length = print("Part 2...", 0);
                        }
                    }
                    SolveProgress::ErrorResult(p) => print_and_copy(
                        p.part.unwrap() as u32,
                        &progress,
                        &p.duration,
                        &config,
                        prev_line_length,
                    ),
                    SolveProgress::Done(p) => {
                        println!("Runtime: {}", fmt_duration(&p.duration));
                        solution_duration = p.duration;
                    }
                    SolveProgress::Error(p) => println!("\nError: {}", p),
                    SolveProgress::Progress(p) => {
                        prev_line_length = print(
                            &format!(
                                "Part {}... ({}) {:.2}%",
                                p.part.unwrap(),
                                fmt_duration(&p.duration),
                                p.value * 100.0
                            ),
                            prev_line_length,
                        );
                    }
                }
            }
        }

        // println!(
        //     "loops: {}, lock: {:?}, sleep: {:?}",
        //     _dbg_loop_count, _dbg_lock_duration, _dbg_sleep_duration
        // );
        solution_duration
    });

    let duration = t.join().unwrap();
    Ok(duration)
}
