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
    let config = Config::init();
    match args {
        Args { add: Some(path) } => add(&config, path),
        Args { add: None } => commit(),
    }
}

fn add(config: &Config, path: String) {}

fn commit() {}
