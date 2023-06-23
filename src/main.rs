mod config;
mod git;

use std::{path::PathBuf, process::Command};

use clap::Parser;
use config::{Config, Syncronizable, HOME_PATH};

/// Simple program to manage dotfiles
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Files to add to the dotfile directory
    #[arg(short, long)]
    add: Option<PathBuf>,
}

fn main() {
    let mut config = Config::init();
    match Args::parse() {
        Args { add: None } => syncronize(&mut config),
        Args { add: Some(path) } => add(&mut config, path),
        _ => {}
    }
}

fn add(config: &mut Config, path: PathBuf) {
    let current_dir = std::env::current_dir().unwrap();
    let full_path = current_dir.join(&path);
    let full_path_str = full_path.to_string_lossy().to_string();
    match !config.file.files.contains(&full_path) {
        true => {
            println!("Adding.. {}", full_path_str);
            config.file.files.push(full_path);
            syncronize(config);
        }
        false => println!("File is already added.."),
    }
}

fn commit() {
    git::commit(HOME_PATH.to_string());
}

fn syncronize(config: &mut Config) {
    config.syncronize(config.home.to_string());
    commit();
}

fn reset() {}
