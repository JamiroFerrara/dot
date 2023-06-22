mod config;

use anyhow::Result;
use clap::Parser;
use config::Config;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long)]
    add: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mut config = Config::init();
    match args {
        Args { add: Some(path) } => add(&mut config, path),
        Args { add: None } => commit(),
    }
}

fn add(config: &mut Config, path: String) {
    config.dotfiles.push(path);
    config.file.serialize();
}

fn commit() {}
