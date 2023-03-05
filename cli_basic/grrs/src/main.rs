#![allow(unused)]

use clap::Parser;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser, Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    let file = File::open(&args.path)?;
    // println!("{:?}", args)
    // let content = std::fs::read_to_string(&args.path).expect("Could not read file");
    let content = BufReader::new(file);

    for line in content.lines() {
        let line = line.unwrap();
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}
