use crate::core::solution::*;
use regex::Regex;

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
    let info = T::as_type().info;
    let file_path = format!("../input/day{}.txt", info.day_str());
    let input =
        std::fs::read_to_string(&file_path).expect(&format!("reading input file '{}'", &file_path));
    return setup::<T>(&input);
}
