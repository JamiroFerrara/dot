mod constants;
mod file;

use anyhow::Result;
use constants::HOME_PATH;
use file::ConfigurationFile;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug)]
pub struct Config {
    pub home: String,
    pub dotfiles: Vec<String>,
    pub file: ConfigurationFile,
}

impl Config {
    pub fn init() -> Config {
        let mut files: Vec<String> = Vec::new();
        files.push("/home/stiwie/.config/test.txt".to_string());
        let mut file = ConfigurationFile::new(files);

        let config_path = get_config_path();
        match config_exists() {
            true => file.deserialize(config_path),
            false => file.serialize(config_path),
        }

        return Config {
            dotfiles: file.files.clone(),
            home: HOME_PATH.to_string(),
            file,
        };
    }
}

pub fn get_config_path() -> String {
    HOME_PATH.to_string() + "/config.json"
}

pub fn config_exists() -> bool {
    let file_path = get_config_path();
    let path = Path::new(&file_path);
    match directory_exists() {
        true => path.exists(),
        false => {
            let file_path = HOME_PATH.to_string();
            fs::create_dir(file_path)
                .expect("Something when wrong when creating the directory path..");
            config_exists()
        }
    }
}

pub fn directory_exists() -> bool {
    let file_path = HOME_PATH.to_string();
    let path = Path::new(&file_path);
    path.exists()
}
