use aoc::util::*;
use aoc_cli::{args::*, config::*, scaffold, solve, timing};
use aoc_ui;
use clap::Parser;
use clearscreen;
fn main() {
    _ = clearscreen::clear();
    aoc_ui::char_image::print_line("AOC 2022", '#', ' ');
    println!("--- Advent of Code 2022 CLI by sanraith ---");

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
    match args.mode {
        Some(Command::Scaffold { year, days, inputs }) => scaffold(&config, year, days, inputs),
        Some(Command::Solve { days }) => {
            let days_str = days
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            println!("Solving days: {}", days_str);
            days.iter()
                .for_each(|d| solve::run_solution(&config, 2021, *d).unwrap())
        }
        None => {
            _ = aoc_ui::entry::main();
            // println!("Solving all days of all years...");
            // solve::run_solutions(&config).unwrap();
        }
    }
}

fn scaffold(config: &Config, year: Option<i32>, days: Vec<u32>, inputs: bool) {
    if inputs {
        scaffold::scaffold_missing_inputs(config);
    } else {
        let year = match year {
            Some(year) => year,
            None => timing::latest_aoc_date().year,
        };

        match days.len() {
            1.. => days
                .into_iter()
                .for_each(|day| scaffold::scaffold_day(config, year, day)),
            _ => scaffold::scaffold_day(config, year, timing::latest_aoc_date().day),
        };
    }
}
