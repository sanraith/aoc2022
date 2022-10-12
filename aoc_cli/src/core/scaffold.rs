use crate::config::{self, Config};
use aoc::util::day_str;
use regex::Regex;
use std::borrow::Cow;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use url::Url;

const CACHE_DIR: &'static str = ".cache";
const BASE_URL: &'static str = "https://adventofcode.com/";

const SOLUTION_DIR: &'static str = "aoc_lib/src/solutions/";
const SOLUTION_TEMPLATE_PATH: &'static str = "aoc_lib/templates/day__DAY_STR__.rs.template";
const DAY_PLACEHOLDER: &'static str = "__DAY__";
const YEAR_PLACEHOLDER: &'static str = "__YEAR__";
const TITLE_PLACEHOLDER: &'static str = "__TITLE__";
const DAY_STR_PLACEHOLDER: &'static str = "__DAY_STR__";

#[derive(Default)]
struct PuzzleInfo {
    title: String,
    year: i32,
    day: u32,
    day_str: String,
}

pub fn scaffold_day(year: i32, day: u32) {
    let session_key = get_session_key();
    println!("Scaffolding for year {} day {}... ", year, day);
    let puzzle_info = PuzzleInfo {
        year,
        day,
        day_str: day_str(day),
        ..Default::default()
    };

    let puzzle_url = format!("{year}/day/{day}");
    let input_url = format!("{year}/day/{day}/input");
    let _puzzle = request_cached(&puzzle_url, &session_key).unwrap();
    let _input = request_cached(&input_url, &session_key).unwrap();

    generate_solution_file(&puzzle_info).unwrap();

    println!("Ok.");
}

fn get_session_key() -> String {
    let config_path = config::DEFAULT_CONFIG_PATH;
    let config = match Config::load_from_file(config_path) {
        Ok(config) => config,
        Err(_) => {
            println!("Could not load session_key from '{}'", &config_path);
            print!("Please provide your AOC session key: ");
            std::io::stdout().flush().unwrap();

            let mut session_key = String::new();
            let stdin = std::io::stdin();
            stdin
                .read_line(&mut session_key)
                .expect("reading session_key from user");

            let config = Config {
                session_key: session_key.trim().to_owned(),
            };
            config
                .save_to_file(config_path)
                .expect("saving config file");

            config
        }
    };

    config.session_key
}

fn request_cached(sub_url: &str, _session_key: &str) -> Result<String, Box<dyn Error>> {
    let cached_file_name = format!("{}.txt", sub_url.replace("/", "_"));
    let cached_file_path = Path::new(CACHE_DIR).join(cached_file_name);
    let url = Url::parse(BASE_URL)?.join(sub_url)?;
    if let Ok(s) = fs::read_to_string(&cached_file_path) {
        println!(
            "Using cached file '{}' for {}",
            &cached_file_path.to_str().unwrap(),
            &url
        );
        return Ok(s);
    }

    println!("Requesting: {}", &url);
    let contents = ureq::get(&url.to_string())
        .set("cookie", &format!("session={_session_key};"))
        .call()?
        .into_string()?;

    println!(
        "Storing response in cache: {}",
        &cached_file_path.to_str().unwrap()
    );
    fs::create_dir_all(&cached_file_path.parent().unwrap())
        .expect("Unable to create cache directory");
    let mut f = File::create(&cached_file_path).expect("Unable to create cache file");
    f.write_all(contents.as_bytes())
        .expect("Unable to write data to cache file");

    Ok(contents)
}

fn generate_solution_file(puzzle_info: &PuzzleInfo) -> Result<(), Box<dyn Error>> {
    let target_file_path = match Path::new(&replace_placeholder(
        SOLUTION_TEMPLATE_PATH.to_owned(),
        DAY_STR_PLACEHOLDER,
        &puzzle_info.day_str,
    ))
    .with_extension("")
    .file_name()
    {
        Some(x) => x.to_str().unwrap().to_owned(),
        None => return Err("Target path invalid".into()),
    };
    let target_file_path = Path::new(SOLUTION_DIR).join(&target_file_path);
    println!("Scaffolding source: {}", target_file_path.to_str().unwrap());

    let contents = fs::read_to_string(SOLUTION_TEMPLATE_PATH)?;
    let contents = replace_placeholder(contents, YEAR_PLACEHOLDER, &puzzle_info.year.to_string());
    let contents = replace_placeholder(contents, DAY_PLACEHOLDER, &puzzle_info.day.to_string());
    let contents = replace_placeholder(contents, DAY_STR_PLACEHOLDER, &puzzle_info.day_str);
    let contents = replace_placeholder(contents, TITLE_PLACEHOLDER, &puzzle_info.title);

    fs::create_dir_all(SOLUTION_DIR)?;
    let mut f = File::create(&target_file_path)?;
    f.write_all(contents.as_bytes())?;

    Ok(())
}

fn replace_placeholder(source: String, placeholder: &str, target: &str) -> String {
    let regex = match Regex::new(&format!("([ \t]*){}", placeholder)) {
        Ok(regex) => regex,
        Err(_) => return source,
    };

    let mut source = source;
    while let Some(captures) = regex.captures(&source) {
        let indent = &captures[1];
        let indented_content = target
            .lines()
            .map(|l| indent.to_string() + l)
            .collect::<Vec<_>>()
            .join("\n");

        match regex.replace(&source, &indented_content) {
            Cow::Borrowed(_) => break,
            Cow::Owned(new) => source = new,
        };
    }

    return source;
}
