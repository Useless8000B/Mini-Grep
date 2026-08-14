use crate::config::Config;
use mini_grep::search;
use std::{env, fs};
mod config;

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = match Config::new(&args) {
        Ok(b) => b,
        Err(e) => {
            println!("Error creating config: {e}");
            std::process::exit(1)
        }
    };

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        std::process::exit(1);
    };
}
