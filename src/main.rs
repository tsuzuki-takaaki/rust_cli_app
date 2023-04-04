// import
mod types;

// namespace
use clap::Parser;
use std::{io::{BufReader, BufRead, BufWriter}, fs::File};
use std::io::Write;
use crate::types::cli::Cli;

fn main() {
  let args = Cli::parse();

  // args.path needs filename extension
  // correct: cargo run hello src/main.rs
  // incorrect: cargo run hello src/main
  let file = BufReader::new(File::open(&args.path).expect("No such file or directory"));
  check_pattern(file, &args)
}

fn check_pattern(file: BufReader<File>, args: &Cli) {
  let stdout = std::io::stdout();
  let mut handle = BufWriter::new(stdout.lock());
  for line in file.lines() {
    // as_ref is so usable like this situation
    if line.as_ref().unwrap().contains(&args.pattern) {
      writeln!(handle, "{}", line.unwrap()).unwrap();
    }
  }
}
