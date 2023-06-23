mod config;

use std::path::PathBuf;

use clap::Parser;
use config::{Config, Syncronizable};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long)]
    add: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let mut config = Config::init();
    match args {
        Args { add: Some(path) } => add(&mut config, path),
        Args { add: None } => commit(),
    }
}

fn add(config: &mut Config, path: PathBuf) {
    let current_dir = std::env::current_dir().unwrap();
    let full_path = current_dir.join(&path);
    let full_path_str = full_path.to_string_lossy().to_string();
    println!("Adding.. {}", full_path_str);
    if !config.file.files.contains(&full_path) {
        config.file.files.push(full_path);
    }
    syncronize(config);
}

fn syncronize(config: &mut Config) {
    config.file.serialize();
    config.syncronize(config.home.to_string());
}

fn commit() {
    //TODO: Implement github repo creation an commit
}
