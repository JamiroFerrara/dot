use std::{fs::File, io::Read, os::unix::prelude::FileExt};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigurationFile {
    pub files: Vec<String>,
}

impl ConfigurationFile {
    pub fn new() -> ConfigurationFile {
        ConfigurationFile { files: Vec::new() }
    }

    pub fn serialize(&self, path: String) {
        let res = serde_json::to_string_pretty(self).unwrap();
        let file = File::create(path).expect("Failed to create file..");
        file.write_all_at(res.as_bytes(), 0)
            .expect("Failed to write configuration file..");
    }

    pub fn deserialize(&mut self, path: String) {
        let mut file = File::open(path).expect("Failed to open file..");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to open file..");
        let res: ConfigurationFile = serde_json::from_str(&contents).unwrap();
        *self = res
    }
}
