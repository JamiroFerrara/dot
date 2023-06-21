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
    commit: Option<String>,
}

fn main() {
    let args = Args::parse();
    let config = Config::init();
    match args {
        Args { commit: None } => add(&config.unwrap()),
        Args { commit: Some(path) } => commit(path),
    }
}

fn add(config: &Config) {}

fn commit(path: String) {
    println!("Committing");
}
