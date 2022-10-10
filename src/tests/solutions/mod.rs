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
    let _file_name = format!("day{}.txt", T::day_str().unwrap());
    let input = ""; // TODO read from file
    return setup::<T>(input);
}

mod day01_test;
