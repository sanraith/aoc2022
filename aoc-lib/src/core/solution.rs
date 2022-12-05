use crate::util::{GenericResult, YearDay};
use regex::Regex;
use std::{cell::RefCell, error::Error, fmt, str::FromStr};

pub type SolutionResult = GenericResult<String>;

/// Indicates that the solution method is not implemented yet.
#[derive(Debug, Clone)]
pub struct NotImplementedError;
impl Error for NotImplementedError {}
impl fmt::Display for NotImplementedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "method is not implemented")
    }
}

pub type Title = SolutionInfo;
pub struct SolutionInfo {
    pub year: i32,
    pub day: u32,
    pub title: String,
}
impl SolutionInfo {
    pub fn new(year: i32, day: u32, title: &'static str) -> Self {
        SolutionInfo {
            year,
            day,
            title: title.to_owned(),
        }
    }

    pub fn year_day(&self) -> YearDay {
        YearDay {
            year: self.year,
            day: self.day,
        }
    }
}

pub trait ProgressHandler {
    fn on_progress(&mut self, _value: f32) {}
}
pub struct NopOnProgress;
impl ProgressHandler for NopOnProgress {}

pub struct Context {
    pub raw_input: String,
    pub progress_handler: RefCell<Box<dyn ProgressHandler>>,
}
impl Default for Context {
    fn default() -> Self {
        Self {
            raw_input: Default::default(),
            progress_handler: RefCell::new(Box::new(NopOnProgress)),
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
            v if v >= 0.0 && v <= 1.0 => Ok(self.progress_handler.borrow_mut().on_progress(v)),
            _ => Err(format!("Invalid progress value: {}", value)),
        }
    }
}

pub struct SolutionType {
    pub info: SolutionInfo,
    ctor: fn() -> Box<dyn Solution>,
}
impl SolutionType {
    pub fn create_new(&self) -> Box<dyn Solution> {
        (self.ctor)()
    }
}

pub trait SolutionStatic
where
    Self: Solution + Default + 'static,
{
    fn new() -> Self {
        Self::default()
    }

    fn as_type() -> SolutionType {
        SolutionType {
            info: Self::new().info(),
            ctor: || Box::new(Self::new()),
        }
    }
}
impl<T: Solution + Default + 'static> SolutionStatic for T {}

pub trait Solution {
    // This is an instance method to satisfy object safety and to require only 1 impl block for implementers.
    fn info(&self) -> SolutionInfo;
    fn part1(&mut self, ctx: &Context) -> SolutionResult;
    fn part2(&mut self, ctx: &Context) -> SolutionResult;

    #[allow(unused_variables)]
    fn init(&mut self, ctx: &Context) -> GenericResult {
        Ok(())
    }
}
