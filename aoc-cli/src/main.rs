use aoc::{
    solutions::{self},
    util::*,
};
use aoc_cli::{args::*, config::*, scaffold, solve, timing};
use aoc_ui;
use clap::Parser;
use itertools::Itertools;
use std::time::Duration;

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
        Some(Command::Day12Extra) => extras::day12_extra(),
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
    let total_duration = days.iter().fold(Duration::default(), |a, yd| {
        a + solve::run_solution(&config, yd.year, yd.day).unwrap()
    });

    if days.len() > 1 {
        println!(
            "\n{} solutions run for {} in total.",
            days.len(),
            fmt_duration(&total_duration)
        );
    }
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

mod extras {
    use aoc::{inputs, solution::Context, solutions::year2022, util::YearDay};
    use itertools::Itertools;
    use std::{fs::File, io::Write};

    pub fn day12_extra() {
        let ctx = Context {
            raw_input: inputs::get(&YearDay::new(2022, 12)).unwrap().to_owned(),
            ..Default::default()
        };

        let map = year2022::day12::parse_height_map(&ctx).unwrap();
        let map = map
            .tiles
            .into_iter()
            .map(|l| l.into_iter().map(|x| x as f32 + 0.0).collect_vec())
            .collect_vec();

        let map = scale(map, 4);
        let map = avg(map);
        let map = avg(map);

        let out = map.iter().map(|l| l.iter().join(" ")).join("\n");
        let mut file = File::create("extras/height_map_avg4.dat").unwrap();
        file.write_all(out.as_bytes()).unwrap();
    }

    fn scale(map: Vec<Vec<f32>>, count: usize) -> Vec<Vec<f32>> {
        map.into_iter()
            .flat_map(|l| vec![l.clone(); count])
            .map(|l| l.into_iter().flat_map(|x| vec![x; count]).collect_vec())
            .collect_vec()
    }

    fn avg(map: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
        let width = map.get(0).unwrap().len();
        let height = map.len();
        let mut avg_map = vec![vec![0.0; width]; height];
        itertools::iproduct!(1..width - 1, 1..height - 1).for_each(|(x, y)| {
            avg_map[y][x] =
                (map[y][x] + map[y - 1][x] + map[y][x + 1] + map[y + 1][x] + map[y][x - 1]) as f32
                    / 5.0;
        });
        return avg_map;
    }
}
