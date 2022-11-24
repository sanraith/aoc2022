use crate::solution::SolutionInfo;
use std::path::PathBuf;

const INPUT_PATH: &'static str = "aoc-lib/input";

pub fn year_directory_name(year: i32) -> String {
    format!("year{}", year)
}

pub fn input_file_name(info: &SolutionInfo) -> String {
    format!("day{}.txt", info.day_str())
}

pub fn input_file_path(info: &SolutionInfo) -> String {
    PathBuf::from_iter([
        INPUT_PATH,
        &year_directory_name(info.year),
        &input_file_name(info),
    ])
    .to_str()
    .unwrap()
    .to_owned()
}
