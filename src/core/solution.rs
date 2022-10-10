use regex::Regex;
use std::{error::Error, str::FromStr};

pub type SolutionResult = Result<String, Box<dyn Error>>;

pub struct Context {
    pub raw_input: String,
    pub on_progress: fn(f32) -> (),
}
impl Default for Context {
    fn default() -> Self {
        Self {
            raw_input: Default::default(),
            on_progress: |_| (),
        }
    }
}
impl Context {
    /// Provides a cleaned-up version of raw_input with the following modifications:
    /// - Convert \r\n to \n
    /// - Remove leading and trailing whitespace lines
    pub fn input(&self) -> String {
        let newline_re = Regex::new(r"\r\n").unwrap();
        let trim_re = Regex::new(r"^(?:\s*\n)+|(?:\n\s*)+$").unwrap();
        let input = newline_re.replace_all(&self.raw_input, "\n");
        let input = trim_re.replace_all(&input, "");

        input.into()
    }

    /// Parses each line of the input as a value of type T.
    pub fn input_values<T>(&self) -> Result<Vec<T>, <T as FromStr>::Err>
    where
        T: FromStr,
    {
        self.input()
            .lines()
            .map(|x| x.parse::<T>())
            .collect::<Result<Vec<_>, _>>()
    }

    /// Updates the current progress percentage.
    /// value range: 0..1
    pub fn progress(&self, value: f32) -> Result<(), String> {
        match value {
            v if v >= 0.0 && v <= 1.0 => Ok((self.on_progress)(v)),
            _ => Err(format!("Invalid progress value: {}", value)),
        }
    }
}

pub trait Solution
where
    Self: Default,
{
    fn new() -> Self {
        Self::default()
    }

    fn day_str() -> Option<String> {
        Some(
            Regex::new(r"(?:::Day)(\d+)")
                .unwrap()
                .captures(std::any::type_name::<Self>())?
                .get(1)?
                .as_str()
                .to_owned(),
        )
    }

    fn day_number() -> Option<u8> {
        Self::day_str()?.parse::<u8>().ok()
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult;
    fn part2(&mut self, ctx: &Context) -> SolutionResult;
}
