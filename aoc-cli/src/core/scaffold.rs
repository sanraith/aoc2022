use crate::config::Config;
use aoc::core::file_util;
use aoc::solution::SolutionInfo;
use aoc::util::{day_str, GenericResult, MsgError};
use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use std::borrow::Cow;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use url::Url;

const CACHE_DIR: &'static str = ".cache";
const BASE_URL: &'static str = "https://adventofcode.com/";

const SOLUTION_DIR: &'static str = "aoc-lib/src/solutions/";
const SOLUTION_TEMPLATE_PATH: &'static str =
    "aoc-lib/templates/solution/day__DAY_STR__.rs.template";

const TEST_DIR: &'static str = "aoc-lib/src/tests/solutions/";
const TEST_TEMPLATE_PATH: &'static str = "aoc-lib/templates/test/day__DAY_STR___test.rs.template";
const INPUT_TEMPLATE_PATH: &'static str = "aoc-lib/templates/input/day__DAY_STR__.txt.template";

const DAY_PLACEHOLDER: &'static str = "__DAY__";
const YEAR_PLACEHOLDER: &'static str = "__YEAR__";
const TITLE_PLACEHOLDER: &'static str = "__TITLE__";
const DAY_STR_PLACEHOLDER: &'static str = "__DAY_STR__";
const PUZZLE_INPUT_PLACEHOLDER: &'static str = "__PUZZLE_INPUT__";
const EXAMPLE_INPUT_PLACEHOLDER: &'static str = "__EXAMPLE_INPUT__";
const EXAMPLE_PART_1_RESULT_PLACEHOLDER: &'static str = "__EXAMPLE_PART_1_RESULT__";

#[derive(Default)]
struct PuzzleInfo {
    title: String,
    year: i32,
    day: u32,
    day_str: String,
    puzzle_input: String,
    example_input: String,
    example_part1_result: String,
}
impl<'a> From<&PuzzleInfo> for SolutionInfo {
    fn from(p: &PuzzleInfo) -> Self {
        SolutionInfo {
            year: p.year,
            day: p.day,
            title: p.title.to_owned(),
        }
    }
}

trait JoinText {
    fn join(self, sep: &str) -> String;
}
impl JoinText for scraper::element_ref::Text<'_> {
    fn join(self, sep: &str) -> String {
        self.collect::<Vec<_>>().join(sep)
    }
}

pub fn scaffold_day(config: &Config, year: i32, day: u32) {
    let session_key = match &config.session_key {
        Some(key) => key.to_owned(),
        None => {
            println!("Please provide your session key in aoc_config.ini!");
            return;
        }
    };

    println!("Scaffolding for year {} day {}... ", year, day);
    let mut puzzle_info = PuzzleInfo {
        year,
        day,
        day_str: day_str(day),
        ..Default::default()
    };
    parse_puzzle_info(&mut puzzle_info, &session_key);

    let get_dir = |base_dir: &str| -> String {
        PathBuf::from_iter([base_dir, &file_util::year_directory_name(year)])
            .to_str()
            .unwrap()
            .to_owned()
    };

    let solution_dir = get_dir(SOLUTION_DIR);
    let test_dir = get_dir(TEST_DIR);
    let fs = generate_file(&puzzle_info, SOLUTION_TEMPLATE_PATH, &solution_dir).unwrap();
    let ft = generate_file(&puzzle_info, TEST_TEMPLATE_PATH, &test_dir).unwrap();
    let fi = generate_file(
        &puzzle_info,
        INPUT_TEMPLATE_PATH,
        PathBuf::from(file_util::input_file_path(&(&puzzle_info).into()))
            .parent()
            .unwrap()
            .to_str()
            .unwrap(),
    )
    .unwrap();

    if let Some(editor_name) = &config.editor_after_scaffold {
        println!("Opening scaffolded files in {}...", editor_name);
        Command::new("cmd")
            .args(["/C", &editor_name, &fi, &ft, &fs])
            .output()
            .expect("open scaffolded files in editor");
    }

    println!("Re-building to generate indexes...");
    Command::new("cargo")
        .args(["build", "-p", "aoc-lib"])
        .output()
        .expect("builds without errors");

    println!("Ok.");
}

fn parse_puzzle_info(puzzle_info: &mut PuzzleInfo, session_key: &str) {
    let input_url = format!("{}/day/{}/input", puzzle_info.year, puzzle_info.day);
    puzzle_info.puzzle_input = request_cached(&input_url, session_key).unwrap_or(String::default());

    let description_url = format!("{}/day/{}", puzzle_info.year, puzzle_info.day);
    let html = request_cached(&description_url, session_key).unwrap_or(String::default());
    let html = Html::parse_document(&html);

    let title_re = Regex::new(r".*: (.*) ---").unwrap();
    puzzle_info.title = html
        .select(&Selector::parse("h2").unwrap())
        .next()
        .map_or(None, |elem| {
            title_re
                .captures(&elem.text().join(" "))
                .map_or(None, |c| Some(c.get(1).unwrap().as_str().to_owned()))
        })
        .unwrap_or(String::default());

    // Take the first block that has an 'example' sentence before it, or the first one without if none found
    let example_input_candidates = html
        .select(&Selector::parse("article:first-of-type pre code").unwrap())
        .filter_map(|elem| {
            elem.parent()
                .unwrap()
                .prev_siblings()
                .filter(|x| x.has_children())
                .next()
                .and_then(ElementRef::wrap)
                .and_then(|x| Some(x.text().join(" ")))
                .and_then(|x| Some((x.to_lowercase().contains("example"), elem.text().join(" "))))
        })
        .collect::<Vec<_>>();

    puzzle_info.example_input = example_input_candidates
        .iter()
        .filter(|x| x.0)
        .next()
        .map_or(
            example_input_candidates
                .get(0)
                .map_or(String::default(), |x| x.1.to_owned()),
            |x| x.1.to_owned(),
        );

    // Take the last block that does not end with a question
    let ends_with_question_re = Regex::new(r"^.*\?\s*$").unwrap();
    puzzle_info.example_part1_result = html
        .select(&Selector::parse("article:first-of-type em").unwrap())
        .map(|e| e.text().join(" "))
        .filter(|e| ends_with_question_re.captures(e).is_none())
        .last()
        .map_or(String::default(), |x| x.trim().to_owned());
}

fn request_cached(sub_url: &str, session_key: &str) -> GenericResult<String> {
    let cached_file_name = format!("{}.txt", sub_url.replace("/", "_"));
    let cached_file_path = Path::new(CACHE_DIR).join(cached_file_name);
    let url = Url::parse(BASE_URL)?.join(sub_url)?;
    if let Ok(s) = fs::read_to_string(&cached_file_path) {
        println!(
            "Using cached '{}' instead of {}",
            &cached_file_path.to_str().unwrap(),
            &url
        );
        return Ok(s);
    }

    println!("Requesting: {}", &url);
    let contents = ureq::get(&url.to_string())
        .set("cookie", &format!("session={session_key};"))
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

fn generate_file(
    puzzle_info: &PuzzleInfo,
    template_path: &str,
    out_dir: &str,
) -> GenericResult<String> {
    let mut contents = fs::read_to_string(template_path)?;
    replace_placeholders(&mut contents, &puzzle_info);

    let (mut file, path) = create_file(puzzle_info, template_path, out_dir)?;
    file.write_all(contents.as_bytes())?;
    Ok(path.to_str().unwrap().to_owned())
}

fn create_file(
    puzzle_info: &PuzzleInfo,
    template_path: &str,
    target_dir: &str,
) -> GenericResult<(File, PathBuf)> {
    let target_file_name = match Path::new(replace_placeholder(
        &mut template_path.to_owned(),
        DAY_STR_PLACEHOLDER,
        &puzzle_info.day_str,
    ))
    .with_extension("")
    .file_name()
    {
        Some(x) => x.to_str().unwrap().to_owned(),
        None => return Err(MsgError("Target path invalid").into()),
    };

    let target_file_path = Path::new(target_dir).join(&target_file_name);
    println!("Scaffolding: {}", target_file_path.to_str().unwrap());
    fs::create_dir_all(
        target_file_path
            .parent()
            .ok_or(MsgError("create directory for file"))?,
    )?;
    let file = File::create(&target_file_path)?;

    Ok((file, target_file_path))
}

fn replace_placeholders(contents: &mut String, puzzle_info: &PuzzleInfo) {
    let formatted_input = match puzzle_info.example_input.lines().count() {
        2.. => Cow::from(format!("\n{}", puzzle_info.example_input)),
        _ => Cow::from(&puzzle_info.example_input),
    };

    let replacements = [
        (YEAR_PLACEHOLDER, &puzzle_info.year.to_string() as &str),
        (DAY_PLACEHOLDER, &puzzle_info.day.to_string()),
        (TITLE_PLACEHOLDER, &puzzle_info.title),
        (DAY_STR_PLACEHOLDER, &puzzle_info.day_str),
        (EXAMPLE_INPUT_PLACEHOLDER, &formatted_input),
        (
            EXAMPLE_PART_1_RESULT_PLACEHOLDER,
            &puzzle_info.example_part1_result,
        ),
        (PUZZLE_INPUT_PLACEHOLDER, &puzzle_info.puzzle_input),
    ];

    for (placeholder, target) in replacements {
        replace_placeholder(contents, placeholder, target);
    }
}

fn replace_placeholder<'a>(
    source: &'a mut String,
    placeholder: &str,
    target: &str,
) -> &'a mut String {
    let regex = match Regex::new(&format!("([ \t]*){}", placeholder)) {
        Ok(regex) => regex,
        Err(_) => return source,
    };

    while let Some(captures) = regex.captures(&source) {
        let indent = &captures[1];
        let indented_content = target
            .lines()
            .map(|l| indent.to_string() + l)
            .collect::<Vec<_>>()
            .join("\n");

        match regex.replace(&source, &indented_content) {
            Cow::Borrowed(_) => break,
            Cow::Owned(new) => *source = new,
        };
    }

    source
}
