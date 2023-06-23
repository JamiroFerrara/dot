use std::{
    fs,
    path::{Path, PathBuf},
};

pub trait Syncronizable {
    fn syncronize(&self, destination: String);
}

impl Syncronizable for Vec<PathBuf> {
    fn syncronize(&self, destination: String) {
        for path in self {
            let file_name = path.file_name().expect("Error getting file name..");
            let full_dest = destination.clone() + "/" + file_name.to_str().unwrap();

            match Path::new(&full_dest).exists() {
                true => {
                    fs::copy(&full_dest, &path)
                        .expect("There was an error when syncronizing the files..");
                    println!("Syncronizing.. {}", full_dest);
                }
                false => {
                    fs::copy(&path, &full_dest)
                        .expect("There was an error when syncronizing the files..");
                    println!("Syncronizing.. {}", path.display());
                }
            }
        }
    }
}
