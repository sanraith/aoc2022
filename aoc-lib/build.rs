use regex::Regex;
use std::{borrow::Cow, error::Error, fs, io::Write, path::PathBuf};

type GenericResult<T> = Result<T, Box<dyn Error>>;

const MODULE_DEFINITIONS_PLACEHOLDER: &'static str = "__MODULE_DEFINITIONS__";
const SOLUTION_TYPE_LIST_PLACEHOLDER: &'static str = "__SOLUTION_TYPE_LIST__";
const SOLUTION_TYPE_LIST_APPEND_PLACEHOLDER: &'static str = "__SOLUTION_TYPE_LIST_APPEND__";
const MODULE_NAME_PLACEHOLDER: &'static str = "__MODULE_NAME__";
const RE_EXPORTS_PLACEHOLDER: &'static str = "__RE_EXPORTS__";
const INPUT_BYTE_DEFINITIONS_PLACEHOLDER: &'static str = "__INPUT_BYTE_DEFINITIONS__";
const INPUT_LIST_PLACEHOLDER: &'static str = "__INPUT_LIST__";

const INPUT_DIRECTORY: &'static str = "input";
const INPUT_MODULE_TEMPLATE_PATH: &'static str = "templates/inputs.rs.template";
const INPUT_MODULE_PATH: &'static str = "src/inputs.rs";
const SOLUTION_DIRECTORY: &'static str = "src/solutions/";
const SOLUTION_MODULE_TEMPLATE_PATH: &'static str = "templates/solution/mod.rs.template";
const TEST_DIRECTORY: &'static str = "src/tests";
const TEST_MODULE_TEMPLATE_PATH: &'static str = "templates/test/mod.rs.template";
const RELATIVE_MODULE_FILE_NAME: &'static str = "../__MODULE_NAME__.rs";

fn main() {
    println!("cargo:rerun-if-changed={}", SOLUTION_DIRECTORY);
    println!("cargo:rerun-if-changed={}", TEST_DIRECTORY);
    println!("cargo:rerun-if-changed={}", INPUT_DIRECTORY);
    if let Err(e) = generate_modules() {
        eprintln!("Error: {}", e);
    }
}

#[derive(Debug)]
struct Input {
    year: i32,
    day: u32,
    day_str:String,
}

fn generate_modules() -> GenericResult<()> {
    generate_source_module(SOLUTION_DIRECTORY)?;
    generate_test_module(TEST_DIRECTORY)?;
    generate_input_module()
}

fn generate_input_module() -> GenericResult<()> {
    let year_day_re = Regex::new(r"year(\d+).day(\d+).txt$")?;
    let input_files = collect_files_rec(PathBuf::from(INPUT_DIRECTORY))?;
    let inputs = input_files
        .into_iter()
        .filter_map(|f| {
            year_day_re.captures(&f).and_then(|c| {
                Some(Input {
                    year: c.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    day: c.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                    day_str: c.get(2).unwrap().as_str().to_owned()
                })
            })
        })
        .collect::<Vec<_>>();

    println!("{:?}", inputs);

    let input_definition_lines = inputs.iter().map(|x| 
        format!("static INPUT_YEAR_{}_DAY_{}: &'static [u8] = include_bytes!(\"../input/year{}/day{}.txt\");", x.year, x.day_str, x.year, x.day_str)
    ).collect::<Vec<_>>();

    let input_list_lines = inputs.iter().map(|x| 
        format!("map.insert(YearDay::new({}, {}), from_utf8(INPUT_YEAR_{}_DAY_{}));", x.year, x.day, x.year, x.day_str)
    ).collect::<Vec<_>>();

    let mut output = fs::read_to_string(INPUT_MODULE_TEMPLATE_PATH)?;
    replace_placeholder(
        &mut output,
        INPUT_BYTE_DEFINITIONS_PLACEHOLDER,
        &input_definition_lines.join("\n"),
    );
    replace_placeholder(
        &mut output,
        INPUT_LIST_PLACEHOLDER,
        &input_list_lines.join("\n"),
    );

    let path = PathBuf::from(INPUT_MODULE_PATH);
    save_file(path, output)?;

    Ok(())
}

fn collect_files_rec(directory: PathBuf) -> GenericResult<Vec<String>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        if entry.path().is_dir() {
            files.append(&mut collect_files_rec(entry.path())?);
        }
        files.push(entry.path().to_str().unwrap().to_owned());
    }

    Ok(files)
}

fn generate_source_module(solution_dir: &str) -> GenericResult<()> {
    let solution_module_re = Regex::new(r"^(day\S+).rs$")?;
    let struct_name_re = Regex::new(r"^[^/]*pub struct (Day[a-zA-Z0-9_]+)")?;

    let mut module_lines: Vec<String> = Vec::new();
    let mut export_lines: Vec<String> = Vec::new();
    let mut vec_lines: Vec<String> = Vec::new();
    let mut vec_append_lines: Vec<String> = Vec::new();
    for (file_name, mod_name) in collect_modules_from_dir(solution_dir, &solution_module_re)? {
        module_lines.push(format!("pub mod {};", mod_name));

        let contents = fs::read_to_string(format!("{}/{}", solution_dir, &file_name))?;
        for captures in struct_name_re.captures_iter(&contents) {
            let struct_name = &captures[1];
            export_lines.push(format!("pub use {mod_name}::{struct_name};"));
            vec_lines.push(format!("{struct_name}::as_type(),"))
        }
    }

    let directories = fs::read_dir(solution_dir)?
        .filter_map(|x| x.ok())
        .filter(|x| x.path().is_dir());
    for entry in directories {
        let path_str = entry.path().to_str().unwrap().to_owned();
        generate_source_module(&path_str)?;
        let mod_name = entry.file_name().to_str().unwrap().to_owned();
        module_lines.push(format!("pub mod {};", mod_name));
        vec_append_lines.push(format!("list.append(&mut {}::create_list());", mod_name));
    }

    let mut output = fs::read_to_string(SOLUTION_MODULE_TEMPLATE_PATH)?;
    replace_placeholder(
        &mut output,
        MODULE_DEFINITIONS_PLACEHOLDER,
        &module_lines.join("\n"),
    );
    replace_placeholder(
        &mut output,
        RE_EXPORTS_PLACEHOLDER,
        &export_lines.join("\n"),
    );
    replace_placeholder(
        &mut output,
        SOLUTION_TYPE_LIST_PLACEHOLDER,
        &vec_lines.join("\n"),
    );
    replace_placeholder(
        &mut output,
        SOLUTION_TYPE_LIST_APPEND_PLACEHOLDER,
        &vec_append_lines.join("\n"),
    );

    let path = get_module_file_name(solution_dir);
    save_file(path, output)?;

    Ok(())
}

fn generate_test_module(test_dir: &str) -> GenericResult<()> {
    let solution_module_re = Regex::new(r"^(day\S+_test).rs$")?;
    let mut module_lines = collect_modules_from_dir(test_dir, &solution_module_re)?
        .iter()
        .map(|(_, mod_name)| format!("pub mod {};", mod_name))
        .collect::<Vec<_>>();

    let directories = fs::read_dir(test_dir)?
        .filter_map(|x| x.ok())
        .filter(|x| x.path().is_dir());
    for entry in directories {
        let path_str = entry.path().to_str().unwrap().to_owned();
        generate_test_module(&path_str)?;
        let mod_name = entry.file_name().to_str().unwrap().to_owned();
        module_lines.push(format!("pub mod {};", mod_name));
    }

    let file_prefix_regex = Regex::new(r"(.*)\..*").unwrap();
    let files = fs::read_dir(test_dir)?
        .filter_map(|x| x.ok())
        .filter(|x| x.path().is_file());
    for entry in files {
        let file_name = entry.file_name().to_str().unwrap().to_owned();
        let mod_name = file_prefix_regex
            .captures(&file_name)
            .and_then(|x| x.get(1))
            .and_then(|x| Some(x.as_str()))
            .unwrap();
        let new_line = format!("pub mod {};", mod_name);
        if !module_lines.contains(&new_line) {
            module_lines.push(new_line);
        }
    }

    let mut output = fs::read_to_string(TEST_MODULE_TEMPLATE_PATH)?;
    replace_placeholder(
        &mut output,
        MODULE_DEFINITIONS_PLACEHOLDER,
        &module_lines.join("\n"),
    );

    let path = get_module_file_name(test_dir);
    save_file(path, output)?;

    Ok(())
}

fn save_file(path: PathBuf, output: String) -> GenericResult<()> {
    if let Ok(current) = fs::read_to_string(&path) {
        // Do not replace if the only differences are line endings
        if current.replace("\r\n", "\n") == output.replace("\r\n", "\n") {
            return Ok(());
        }
    }

    let mut file = fs::File::create(&path)?;
    file.write_all(output.as_bytes())?;
    Ok(())
}

fn get_module_file_name(module_dir: &str) -> PathBuf {
    let module_name = PathBuf::from(module_dir)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let mut file_name = RELATIVE_MODULE_FILE_NAME.to_owned();
    replace_placeholder(&mut file_name, MODULE_NAME_PLACEHOLDER, &module_name);
    let path = PathBuf::from_iter([module_dir, &file_name]);
    path
}

fn collect_modules_from_dir(
    directory: &str,
    solution_module_re: &Regex,
) -> GenericResult<Vec<(String, String)>> {
    let mut modules = Vec::new();
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        if entry.path().is_dir() {
            continue;
        }

        let file_name = entry.file_name().into_string().unwrap();
        let mod_name = match solution_module_re
            .captures(&file_name)
            .map_or(None, |c| c.get(1))
        {
            Some(m) => m.as_str().to_owned(),
            None => continue,
        };

        modules.push((file_name, mod_name));
    }

    Ok(modules)
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
