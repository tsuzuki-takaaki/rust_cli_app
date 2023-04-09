// import
mod types;

// namespace
use clap::Parser;
use colored::*;
use regex::Regex;
use std::{io::{ BufReader, BufRead, BufWriter }, fs::File};
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

  let re = Regex::new(&args.pattern).unwrap();
  // 91 is BrightRed
  // not using colored because ColoredString is not implemented by Replacer[https://docs.rs/regex/latest/regex/trait.Replacer.html]
  let rep = format!("{}{}{}", "\x1b[91m", args.pattern, "\x1b[0m");

  for line in file.lines() {
    let line = line.unwrap();
    if line.contains(&args.pattern) {
      let result = re.replace_all(&line, &rep);
      let path = &args.path.to_str().unwrap().bright_yellow();
      
      writeln!(handle, "{}: {}", path, result).unwrap();
    }
  }
}
