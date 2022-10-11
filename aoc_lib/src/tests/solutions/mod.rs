use crate::core::solution::*;
use regex::Regex;

pub fn setup<T>(input: &str) -> (T, Context)
where
    T: Solution + SolutionStatic,
{
    // Remove leading empty lines to be able to format input in tests nicer
    let input = Regex::new(r"^(?:\s*\n)+")
        .unwrap()
        .replace(input, "")
        .to_string();
    (
        T::new(),
        Context {
            raw_input: input,
            ..Default::default()
        },
    )
}

pub fn setup_from_file<T>() -> (T, Context)
where
    T: Solution + SolutionStatic,
{
    let info = T::as_type().info;
    let file_path = format!("../input/day{}.txt", info.day_str());
    let input =
        std::fs::read_to_string(&file_path).expect(&format!("reading input file '{}'", &file_path));
    return setup::<T>(&input);
}

// Solution modules
mod day01_test;
