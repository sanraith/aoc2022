use aoc::util::*;
use aoc_cli::{args::*, config::*, scaffold, solve, timing};
use clap::Parser;
use clearscreen;

fn main() {
    _ = clearscreen::clear();
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
        Some(Command::Scaffold { year, days }) => scaffold(&config, year, days),
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
            println!("Default behavior");
            solve::run_solution(&config, 2021, 1).unwrap();
        }
    }
}

fn scaffold(config: &Config, year: Option<i32>, days: Vec<u32>) {
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
