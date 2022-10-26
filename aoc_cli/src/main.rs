use aoc::{solution::*, solutions::*, util::DynError};
use aoc_cli::{
    args::*,
    config::{Config, DEFAULT_CONFIG_PATH},
    scaffold, timing,
};
use arboard::Clipboard;
use clap::Parser;
use clearscreen;
use std::io::{self, Write};

fn main() {
    _ = clearscreen::clear();
    println!("--- Advent of Code 2022 CLI by sanraith ---");

    let config = Config::load_from_file(DEFAULT_CONFIG_PATH)
        .or_else(|_| {
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
        }
        None => {
            println!("Default behavior");
            run_solution(&config);
        }
    }
}

fn run_solution(config: &Config) {
    let ctx = Context {
        raw_input: "123\n456\n789".to_owned(),
        on_progress: |p| {
            print!("\r{:.2}%  ", p * 100.0);
            io::stdout().flush().unwrap();
        },
    };
    let mut solution = Day01::new();
    let result = solution.init(&ctx).and_then(|_| solution.part1(&ctx));
    println!(
        "\n{}",
        result
            .as_ref()
            .map(|x| x.to_owned())
            .unwrap_or_else(|x| format!("Error: {}", x))
    );

    // TODO use last available result
    if config.copy_result_to_clipboard {
        if let Ok(result) = &result {
            let mut clipboard = Clipboard::new().expect("access system clipboard");
            clipboard.set_text(result).expect("write system clipboard");
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
