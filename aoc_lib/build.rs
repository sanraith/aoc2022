use regex::Regex;
use std::{borrow::Cow, error::Error, fs, io::Write};

type GenericErrorResult<T> = Result<T, Box<dyn Error>>;

const MODULE_DEFINITIONS_PLACEHOLDER: &'static str = "__MODULE_DEFINITIONS__";

const SOLUTION_DIRECTORY: &'static str = "src/solutions/";
const SOLUTION_MODULE_PATH: &'static str = "src/solutions.rs";
const SOLUTION_MODULE_TEMPLATE_PATH: &'static str = "templates/solution/solutions.rs.template";

const TEST_DIRECTORY: &'static str = "src/tests/solutions/";
const TEST_MODULE_PATH: &'static str = "src/tests/solutions.rs";
const TEST_MODULE_TEMPLATE_PATH: &'static str = "templates/test/solutions.rs.template";

fn main() {
    println!("cargo:rerun-if-changed={}", SOLUTION_DIRECTORY);
    println!("cargo:rerun-if-changed={}", TEST_DIRECTORY);
    if let Err(e) = generate_modules() {
        eprintln!("Error: {}", e);
    }
}

fn generate_modules() -> GenericErrorResult<()> {
    generate_source_module()?;
    generate_test_module()
}

fn generate_source_module() -> GenericErrorResult<()> {
    let solution_module_re = Regex::new(r"^(day\S+).rs$")?;
    let struct_name_re = Regex::new(r"^[^/]*pub struct (Day[a-zA-Z0-9_]+)")?;

    let mut module_lines: Vec<String> = Vec::new();
    let mut export_lines: Vec<String> = Vec::new();
    let mut vec_lines: Vec<String> = Vec::new();
    for (file_name, mod_name) in collect_modules_from_dir(SOLUTION_DIRECTORY, &solution_module_re)?
    {
        module_lines.push(format!("mod {};", mod_name));

        let contents = fs::read_to_string(format!("{}/{}", SOLUTION_DIRECTORY, &file_name))?;
        for captures in struct_name_re.captures_iter(&contents) {
            let struct_name = &captures[1];
            export_lines.push(format!("pub use {mod_name}::{struct_name};"));
            vec_lines.push(format!("{struct_name}::as_type(),"))
        }
    }

    let mut output = fs::read_to_string(SOLUTION_MODULE_TEMPLATE_PATH)?;
    replace_placeholder(
        &mut output,
        MODULE_DEFINITIONS_PLACEHOLDER,
        &module_lines.join("\n"),
    );
    replace_placeholder(&mut output, "__RE_EXPORTS__", &export_lines.join("\n"));
    replace_placeholder(&mut output, "__SOLUTION_TYPE_LIST__", &vec_lines.join("\n"));

    let mut file = std::fs::File::create(SOLUTION_MODULE_PATH)?;
    file.write_all(output.as_bytes())?;

    Ok(())
}

fn generate_test_module() -> GenericErrorResult<()> {
    let solution_module_re = Regex::new(r"^(day\S+_test).rs$")?;
    let module_lines = collect_modules_from_dir(TEST_DIRECTORY, &solution_module_re)?
        .iter()
        .map(|(_, mod_name)| format!("mod {};", mod_name))
        .collect::<Vec<_>>();

    let mut output = fs::read_to_string(TEST_MODULE_TEMPLATE_PATH)?;
    replace_placeholder(
        &mut output,
        MODULE_DEFINITIONS_PLACEHOLDER,
        &module_lines.join("\n"),
    );

    let mut file = std::fs::File::create(TEST_MODULE_PATH)?;
    file.write_all(output.as_bytes())?;

    Ok(())
}

fn collect_modules_from_dir(
    directory: &str,
    solution_module_re: &Regex,
) -> GenericErrorResult<Vec<(String, String)>> {
    let mut modules = Vec::new();
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
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
