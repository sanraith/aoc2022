use regex::Regex;
use std::{borrow::Cow, error::Error, fs, io::Write};

const SOLUTION_MODULE: &'static str = "src/solutions.rs";
const SOLUTION_DIRECTORY: &'static str = "src/solutions/";
const SOLUTION_MODULE_TEMPLATE: &'static str = "templates/solutions.rs.template";

fn main() {
    println!("cargo:rerun-if-changed={SOLUTION_DIRECTORY}");
    if let Err(e) = generate_module() {
        eprintln!("Error: {}", e);
    }
}

fn generate_module() -> Result<(), Box<dyn Error>> {
    let solution_module_re = Regex::new(r"^(day\S+).rs$")?;
    let struct_name_re = Regex::new(r"pub struct (Day[a-zA-Z0-9]+)")?;

    let mut module_lines: Vec<String> = Vec::new();
    let mut export_lines: Vec<String> = Vec::new();
    let mut vec_lines: Vec<String> = Vec::new();

    for entry in fs::read_dir(SOLUTION_DIRECTORY)? {
        let entry = entry?;
        let file_name = entry.file_name().into_string().unwrap();
        let mod_name = match solution_module_re
            .captures(&file_name)
            .map_or(None, |c| c.get(1))
        {
            Some(m) => m.as_str(),
            None => continue,
        };

        module_lines.push(format!("mod {};", mod_name));

        let contents = fs::read_to_string(format!("{}/{}", SOLUTION_DIRECTORY, &file_name))?;
        for captures in struct_name_re.captures_iter(&contents) {
            let struct_name = &captures[1];
            export_lines.push(format!("pub use {mod_name}::{struct_name};"));
            vec_lines.push(format!("{struct_name}::as_type(),"))
        }
    }

    let output = fs::read_to_string(SOLUTION_MODULE_TEMPLATE)?;
    let output = replace_placeholder(output, "__MODULE_DEFINITIONS__", &module_lines.join("\n"));
    let output = replace_placeholder(output, "__RE_EXPORTS__", &export_lines.join("\n"));
    let output = replace_placeholder(output, "__SOLUTION_TYPE_LIST__", &vec_lines.join("\n"));

    let mut file = std::fs::File::create(SOLUTION_MODULE)?;
    file.write_all(output.as_bytes())?;

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
