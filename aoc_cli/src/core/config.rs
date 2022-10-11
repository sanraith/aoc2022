use ini::Ini;
use std::error::Error;

pub const DEFAULT_CONFIG_PATH: &'static str = "session.ini";

pub struct Config {
    pub session_key: String,
}
impl Config {
    pub fn load_from_file(config_file_path: &str) -> Result<Config, Box<dyn Error>> {
        let conf = Ini::load_from_file(config_file_path)?;
        let section = conf
            .section(None::<String>)
            .ok_or("config file should contain correct section")?;
        let session_key = section
            .get("session_key")
            .ok_or("config does should contain session_key")?;

        Ok(Config {
            session_key: session_key.to_owned(),
        })
    }

    pub fn save_to_file(&self, config_file_path: &str) -> Result<(), Box<dyn Error>> {
        let mut conf = Ini::new();
        conf.with_section(None::<String>)
            .set("session_key", &self.session_key);
        conf.write_to_file(config_file_path)?;

        Ok(())
    }
}
