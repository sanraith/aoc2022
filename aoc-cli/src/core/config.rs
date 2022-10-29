use aoc::util::GenericResult;
use ini::Ini;
use regex::Regex;

pub const DEFAULT_CONFIG_PATH: &'static str = "aoc_config.ini";
pub const KEY_SESSION_KEY: &'static str = "session_key";
pub const KEY_EDITOR_AFTER_SCAFFOLD: &'static str = "editor_after_scaffold";
pub const KEY_COPY_RESULT_TO_CLIPBOARD: &'static str = "copy_result_to_clipboard";

#[derive(Default)]
pub struct Config {
    pub session_key: Option<String>,
    pub editor_after_scaffold: Option<String>,
    pub copy_result_to_clipboard: bool,
}
impl Config {
    pub fn load_from_file(config_file_path: &str) -> GenericResult<Config> {
        let whitespace = Regex::new(r"^\s*$").unwrap();
        let conf = Ini::load_from_file(config_file_path)?;
        let section = conf
            .section(None::<String>)
            .ok_or("config file should contain correct section")?;
        let session_key = section.get(KEY_SESSION_KEY).and_then(|x| match x {
            x if whitespace.is_match(x) => None,
            _ => Some(x.to_owned()),
        });
        let editor_after_scaffold = section
            .get(KEY_EDITOR_AFTER_SCAFFOLD)
            .and_then(|x| match x {
                x if whitespace.is_match(x) => None,
                _ => Some(x.to_owned()),
            });
        let copy_result_to_clipboard = section
            .get(KEY_COPY_RESULT_TO_CLIPBOARD)
            .map(|x| x.to_lowercase() == "true")
            .ok_or("config does should contain copy_result_to_clipboard")?;
        Ok(Config {
            session_key: session_key.to_owned(),
            editor_after_scaffold,
            copy_result_to_clipboard,
        })
    }

    pub fn save_to_file(&self, config_file_path: &str) -> GenericResult {
        let mut conf = Ini::new();
        conf.with_section(None::<String>)
            .set(
                KEY_SESSION_KEY,
                &self
                    .session_key
                    .as_ref()
                    .unwrap_or(&"".to_owned())
                    .to_string(),
            )
            .set(
                KEY_EDITOR_AFTER_SCAFFOLD,
                &self
                    .editor_after_scaffold
                    .as_ref()
                    .unwrap_or(&"".to_owned())
                    .to_string(),
            )
            .set(
                KEY_COPY_RESULT_TO_CLIPBOARD,
                if self.copy_result_to_clipboard {
                    "true"
                } else {
                    "false"
                },
            );
        conf.write_to_file(config_file_path)?;

        Ok(())
    }
}
