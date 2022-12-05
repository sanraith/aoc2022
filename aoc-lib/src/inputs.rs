use crate::util::YearDay;
use include_dir::{include_dir, Dir};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt::Debug;
use std::{collections::HashMap, ffi::OsStr, str::FromStr};

static INPUT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/input");
static INPUT_CACHE: Lazy<HashMap<YearDay, String>> = Lazy::new(|| create_input_map());

fn capture_filename_number<T: FromStr>(re: &Regex, path: Option<&OsStr>) -> T
where
    <T as FromStr>::Err: Debug,
{
    re.captures(path.unwrap().to_str().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap()
}

fn create_input_map() -> HashMap<YearDay, String> {
    let year_re = Regex::new(r"year(\d+)").unwrap();
    let day_re = Regex::new(r"day(\d+).txt").unwrap();
    let mut cache = HashMap::new();
    for dir in INPUT_DIR.dirs() {
        let year = capture_filename_number(&year_re, dir.path().file_name());
        for file in dir.files() {
            let day = capture_filename_number(&day_re, file.path().file_name());
            let yd = YearDay::new(year, day);

            cache.insert(yd, file.contents_utf8().unwrap().to_owned());
        }
    }
    cache
}

pub fn get(year_day: &YearDay) -> Option<&String> {
    INPUT_CACHE.get(year_day)
}
