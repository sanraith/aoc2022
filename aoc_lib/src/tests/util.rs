use crate::core::{file_util, solution::*};
use regex::Regex;
use std::{fs, path::PathBuf};

pub fn assert_result(result: SolutionResult, expected: &str, message: &str) {
    match result {
        Ok(actual) if &actual == expected => (),
        Ok(actual) => {
            eprintln!(
                "Failed on {}\nExpected: \"{}\"\nActual:   \"{}\"\n",
                message, expected, &actual
            );
            panic!();
        }
        // Do not fail test if the tested method is not implemented yet
        Err(err) if err.is::<NotImplementedError>() => (),
        Err(err) => {
            eprintln!(
                "Failed on {}\nExpected: \"{}\"\nError:    {:?}\n          {}\n",
                message, expected, err, err
            );
            panic!();
        }
    };
}

pub fn setup<T>(input: &str) -> (T, Context)
where
    T: Solution + SolutionStatic,
{
    // Remove leading empty lines to be able to format input in tests nicer
    let input = Regex::new(r"^(?:\s*\n)+")
        .unwrap()
        .replace(input, "")
        .to_string();
    let ctx = Context {
        raw_input: input,
        ..Default::default()
    };
    let mut day = T::new();
    day.init(&ctx)
        .expect("solution should initialize without errors");

    (day, ctx)
}

pub fn setup_from_file<T>() -> (T, Context)
where
    T: Solution + SolutionStatic,
{
    let file_path = file_util::input_file_path(&T::as_type().info);
    let input_paths = [vec!["..", &file_path], vec![&file_path]];
    let input = input_paths
        .iter()
        .map(|p| PathBuf::from_iter(p).to_str().unwrap().to_owned())
        .find_map(|p| fs::read_to_string(p).ok())
        .expect(&format!("read input file from {:?}", input_paths));

    return setup::<T>(&input);
}
