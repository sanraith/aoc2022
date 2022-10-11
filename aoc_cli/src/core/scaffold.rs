use crate::config::{self, Config};
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use url::Url;

const CACHE_DIR: &'static str = ".cache";
const BASE_URL: &'static str = "https://adventofcode.com/";

pub fn scaffold_day(year: i32, day: u32) {
    let session_key = get_session_key();
    println!("Scaffolding for year {} day {}... ", year, day);

    let puzzle_url = format!("{year}/day/{day}");
    let input_url = format!("{year}/day/{day}/input");
    let _puzzle = request_cached(&puzzle_url, &session_key).unwrap();
    let _input = request_cached(&input_url, &session_key).unwrap();

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
