use std::{fs::File, io::Read, os::unix::prelude::FileExt};

use serde::{Deserialize, Serialize};

use super::constants::HOME_PATH;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigurationFile {
    pub git_user: String,
    pub git_url: String,
    pub files: Vec<String>,
}

impl ConfigurationFile {
    pub fn new(files: Vec<String>) -> ConfigurationFile {
        let git_url = "https://github.com/JamiroFerrara/.dotfiles".to_string();
        let git_user = "Jamiro Ferrara".to_string();
        ConfigurationFile {
            files,
            git_url,
            git_user,
        }
    }

    pub fn serialize(&self) {
        let path = HOME_PATH.to_string() + "/config.json";
        let res = serde_json::to_string_pretty(self).unwrap();
        let file = File::create(path).expect("Failed to create file..");
        file.write_all_at(res.as_bytes(), 0)
            .expect("Failed to write configuration file..");
    }

    pub fn deserialize(&mut self) {
        let path = HOME_PATH.to_string() + "/config.json";
        let mut file = File::open(path).expect("Failed to open file..");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to open file..");
        let res: ConfigurationFile = serde_json::from_str(&contents).unwrap();
        println!("{}", contents);
        *self = res
    }
}
