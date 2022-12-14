use regex::Regex;
use std::{borrow::Cow, error::Error, fs, io::Write, path::PathBuf};

type GenericResult<T> = Result<T, Box<dyn Error>>;

const MODULE_DEFINITIONS_PLACEHOLDER: &'static str = "__MODULE_DEFINITIONS__";
const SOLUTION_TYPE_LIST_PLACEHOLDER: &'static str = "__SOLUTION_TYPE_LIST__";
const SOLUTION_TYPE_LIST_APPEND_PLACEHOLDER: &'static str = "__SOLUTION_TYPE_LIST_APPEND__";
const RE_EXPORTS_PLACEHOLDER: &'static str = "__RE_EXPORTS__";

const SOLUTION_DIRECTORY: &'static str = "src/solutions/";
const SOLUTION_MODULE_TEMPLATE_PATH: &'static str = "templates/solution/mod.rs.template";
const TEST_DIRECTORY: &'static str = "src/tests/";
const TEST_MODULE_TEMPLATE_PATH: &'static str = "templates/test/mod.rs.template";

fn main() {
    println!("cargo:rerun-if-changed={}", SOLUTION_DIRECTORY);
    println!("cargo:rerun-if-changed={}", TEST_DIRECTORY);
    if let Err(e) = generate_modules() {
        eprintln!("Error: {}", e);
    }
}

fn generate_modules() -> GenericResult<()> {
    generate_source_module(SOLUTION_DIRECTORY)?;
    generate_test_module(TEST_DIRECTORY)
}

fn generate_source_module(solution_dir: &str) -> GenericResult<()> {
    let solution_module_re = Regex::new(r"^(day\S+).rs$")?;
    let struct_name_re = Regex::new(r"\n[^/]*pub struct (Day[a-zA-Z0-9_]+)")?;

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
        .filter(|x| x.path().is_file() && x.path().file_name().unwrap() != "mod.rs");
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
    let path = PathBuf::from_iter([module_dir, "mod.rs"]);
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
        if file_name == "mod.rs" {
            continue;
        }

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
