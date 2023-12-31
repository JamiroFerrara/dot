mod constants;
mod file;
mod syncronizable;

use anyhow::Result;
pub use constants::HOME_PATH;
use file::ConfigurationFile;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

pub use syncronizable::Syncronizable;

#[derive(Debug)]
pub struct Config {
    pub home: String,
    pub file: ConfigurationFile,
}

impl Config {
    pub fn init() -> Config {
        let files: Vec<PathBuf> = Vec::new();
        let mut config = ConfigurationFile::new(files);

        match config_exists() {
            true => config.deserialize(),
            false => config.serialize(),
        }

        return Config {
            home: HOME_PATH.to_string(),
            file: config,
        };
    }
}

pub fn config_exists() -> bool {
    let file_path = HOME_PATH.to_string() + "/config.json";
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

impl Syncronizable for Config {
    fn syncronize(&self, destination: String) {
        self.file.serialize();
        self.file.files.syncronize(destination);
    }
}

pub fn directory_exists() -> bool {
    let file_path = HOME_PATH.to_string();
    let path = Path::new(&file_path);
    path.exists()
}
