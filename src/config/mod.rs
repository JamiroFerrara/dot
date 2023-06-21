mod constants;
mod dotfile;
mod file;

use anyhow::Result;
use constants::HOME_PATH;
use dotfile::Dotfile;
use file::ConfigurationFile;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug)]
pub struct Config {
    pub home: String,
    pub dotfiles: Vec<Dotfile>,
    pub file: ConfigurationFile,
}

impl Config {
    pub fn init() -> Result<Config> {
        let file = load_file()?;

        let mut files: Vec<Dotfile> = Vec::new();
        files.push(Dotfile::new(""));

        return Ok(Config {
            dotfiles: files,
            home: HOME_PATH.to_string(),
            file,
        });
    }
}

pub fn load_file() -> Result<ConfigurationFile> {
    let mut file = ConfigurationFile::new();
    let config_path = get_config_path();
    match config_exists() {
        true => file.deserialize(config_path),
        false => file.serialize(config_path),
    }
    Ok(file)
}

pub fn get_config_path() -> String {
    let path = HOME_PATH.to_string() + "/config.json";
    path
}

pub fn config_exists() -> bool {
    let file_path = get_config_path();
    let path = Path::new(&file_path);
    path.exists()
}
