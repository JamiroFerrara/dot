use std::{fs, path::PathBuf};

pub trait Syncronizable {
    fn syncronize(&self, destination: String);
}

impl Syncronizable for Vec<PathBuf> {
    fn syncronize(&self, destination: String) {
        for path in self {
            //let file_name = path.file_name().expect("Error getting file name..");
            let _ = fs::copy(&path, &destination)
                .map(|_| println!("Copied {} to {}", path.display(), destination));
        }
    }
}
