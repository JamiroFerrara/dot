#[derive(Debug)]
pub struct Dotfile {
    path: String,
}

impl Dotfile {
    pub fn new(path: &str) -> Dotfile {
        return Dotfile {
            path: path.to_string(),
        };
    }
}
