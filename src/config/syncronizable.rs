use std::{fs, path::PathBuf};

pub trait Syncronizable {
    fn syncronize(&self, destination: String);
}

impl Syncronizable for Vec<PathBuf> {
    fn syncronize(&self, destination: String) {
        for path in self {
            let file_name = path.file_name().expect("Error getting file name..");
            let full_dest = destination.clone() + "/" + file_name.to_str().unwrap();
            println!("Copied {} to {}", path.display(), full_dest);
            fs::copy(&path, &full_dest).expect("There was an error when syncronizing the files..");
        }
    }
}
