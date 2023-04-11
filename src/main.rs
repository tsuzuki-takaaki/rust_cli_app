// import
mod types;

// namespace
use clap::Parser;
use colored::*;
use regex::Regex;
use std::{io::{ BufReader, BufRead, BufWriter }, fs::File, path::PathBuf};
use std::io::Write;

use crate::types::cli::Cli;

fn main() {
  let args = Cli::parse();
  let target_path = match &args.path {
    Some(path) => std::env::current_dir().unwrap().join(path),
    None => std::env::current_dir().unwrap()
  };

  find_leaf(&target_path, &args);
}

fn find_leaf(target_path: &PathBuf, args: &Cli) {
  if target_path.is_file() {
    check_pattern(&target_path, args);
  } else {
    for entry in target_path.read_dir().expect("No such directory") {
      find_leaf(&entry.unwrap().path(), args);
    }
  }
}

fn check_pattern(target_path: &PathBuf, args: &Cli) {
  let file = BufReader::new(File::open(target_path).expect("No such file"));

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
      
      writeln!(handle, "{}: {}", target_path.to_str().unwrap().yellow(), result).unwrap();
    }
  }
}
