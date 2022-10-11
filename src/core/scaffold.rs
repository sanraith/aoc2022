use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use url::Url;

const YEAR: i32 = 2021;
const CACHE_DIR: &'static str = ".cache";
const BASE_URL: &'static str = "https://adventofcode.com/";

pub enum Target {
    Days(Vec<u8>),
    NextDay(),
}

pub fn scaffold(target: Target) {
    match target {
        Target::Days(days) => days.iter().for_each(|&x| scaffold_day(x)),
        Target::NextDay() => todo!(),
    };
}

fn scaffold_day(day: u8) {
    println!("Scaffolding for year {} day {}... ", YEAR, day);
    let puzzle_url = format!("{YEAR}/day/{day}");
    let _input_url = format!("{YEAR}/day/{day}/input");
    let _contents = request_cached(&puzzle_url).unwrap();

    println!("Ok.");
}

fn request_cached(sub_url: &str) -> Result<String, Box<dyn Error>> {
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
    let contents = ureq::get(&url.to_string()).call()?.into_string()?;

    println!(
        "Storing response in cache: {}",
        &cached_file_path.to_str().unwrap()
    );
    fs::create_dir_all(&cached_file_path.parent().unwrap())
        .expect("Unable to create cache directory");
    let mut f = File::create(&cached_file_path).expect("Unable to create cache file");
    f.write_all(contents.as_bytes())
        .expect("Unable to write data to cache file");
    drop(f);

    Ok(contents)
}
