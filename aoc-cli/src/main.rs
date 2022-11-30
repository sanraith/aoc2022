use aoc::{solutions, util::*};
use aoc_cli::{args::*, config::*, scaffold, solve, timing};
use aoc_ui;
use clap::Parser;
use clearscreen;
use itertools::Itertools;
fn main() {
    let config = Config::load_from_file(DEFAULT_CONFIG_PATH)
        .or_else(|_| {
            println!(
                "Creating configuration with default values: {}",
                DEFAULT_CONFIG_PATH
            );
            let config = Config::default();
            config.save_to_file(DEFAULT_CONFIG_PATH)?;
            Ok::<_, DynError>(config)
        })
        .expect("config loaded or generated");
    let args = Args::parse();

    _ = clearscreen::clear();
    aoc_ui::char_image::print_line("AOC 2022", '#', ' ');
    println!("--- Advent of Code 2022 CLI by sanraith ---");

    match args.mode {
        Some(Command::Scaffold { year, days, inputs }) => scaffold(&config, year, days, inputs),
        Some(Command::Solve { year, days }) => {
            let year = year.unwrap_or(timing::latest_aoc_date().year);
            let mut days = days
                .into_iter()
                .map(|day| YearDay::new(year, day))
                .collect_vec();

            if days.len() == 0 {
                days.append(
                    &mut solutions::create_map()
                        .keys()
                        .filter(|x| x.year == year)
                        .sorted()
                        .map(|x| x.clone())
                        .collect_vec(),
                );
                if days.len() == 0 {
                    println!("Error: no solution found for {}!", year);
                    return;
                }
            }

            solve_days(config, year, days);
        }
        Some(Command::Ui) => _ = aoc_ui::entry::main(),
        None => {
            if let Some(yd) = solutions::create_map().keys().sorted().rev().next() {
                solve_days(config, yd.year, vec![yd.clone()]);
            } else {
                println!("Error: no solution found!");
                return;
            }
        }
    }
}

fn solve_days(config: Config, year: i32, days: Vec<YearDay>) {
    let days_str = days
        .iter()
        .map(|x| x.day.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    println!("Solving days for {}: {}", year, days_str);
    days.iter()
        .for_each(|yd| solve::run_solution(&config, yd.year, yd.day).unwrap())
}

fn scaffold(config: &Config, year: Option<i32>, days: Vec<u32>, inputs: bool) {
    if inputs {
        scaffold::scaffold_inputs(config);
    } else {
        let year = match year {
            Some(year) => year,
            None => timing::latest_aoc_date().year,
        };

        match days.len() {
            1.. => {
                _ = scaffold::scaffold_days(
                    config,
                    days.into_iter()
                        .map(|day| YearDay::new(year, day))
                        .collect_vec(),
                )
            }
            _ => {
                _ = scaffold::scaffold_days(
                    config,
                    vec![YearDay::new(year, timing::latest_aoc_date().day)],
                )
            }
        };
    }
}
