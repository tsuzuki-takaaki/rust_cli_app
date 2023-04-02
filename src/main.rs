// import
mod types;

// namespace
use clap::Parser;
use std::{io::{BufReader, BufRead}, fs::File};
use crate::types::cli::Cli;

fn main() {
  let args = Cli::parse();

  // args.path needs filename extension
  // correct: cargo run hello src/main.rs
  // incorrect: cargo run hello src/main
  let file = BufReader::new(File::open(&args.path).expect("the file is not found"));
  check_pattern(file, &args)
}

fn check_pattern(file: BufReader<File>, args: &Cli) {
  for line in file.lines() {
    // as_ref is so usable like this situation
    if line.as_ref().unwrap().contains(&args.pattern) {
      println!("{}", line.unwrap());
    }
  }
}
