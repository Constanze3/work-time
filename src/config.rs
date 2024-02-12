use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{env, fs, io};

pub fn default_config_path() -> Result<PathBuf, io::Error> {
    let mut dir = env::current_exe()?;
    dir.pop();
    dir.push("config.json");

    Ok(dir)
}

pub fn create_default_config<T: Into<PathBuf>>(path: T) -> Result<Config, io::Error> {
    let path: PathBuf = path.into();

    let mut output_folder = path.clone();
    output_folder.pop();

    let config = Config {
        client_id: String::from("user"),
        output_folder,
    };

    let data = serde_json::to_string_pretty(&config).unwrap();
    fs::write(path, data)?;

    Ok(config)
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub client_id: String,
    pub output_folder: PathBuf,
}

impl Config {
    pub fn load<T: AsRef<Path>>(path: T) -> Result<Config, io::Error> {
        let file = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&file)?;

        Ok(config)
    }
}
