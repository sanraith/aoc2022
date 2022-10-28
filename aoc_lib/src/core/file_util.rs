use crate::solution::SolutionInfo;
use std::path::PathBuf;

pub const INPUT_PATH: &'static str = "input";

pub fn input_file_name(info: &SolutionInfo) -> String {
    format!("day{}.txt", info.day_str())
}

pub fn input_file_path(info: &SolutionInfo) -> String {
    PathBuf::from_iter([
        INPUT_PATH,
        &format!("year{}", info.year),
        &input_file_name(info),
    ])
    .to_str()
    .unwrap()
    .to_owned()
}
