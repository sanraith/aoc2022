use crate::api::solution::*;
use regex::Regex;

pub fn setup<T>(input: &str) -> (T, Context)
where
    T: Solution,
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
    T: Solution,
{
    let file_path = format!("input/day{}.txt", T::day_str().unwrap());
    let input = std::fs::read_to_string(file_path).unwrap();
    return setup::<T>(&input);
}

mod day01_test;
