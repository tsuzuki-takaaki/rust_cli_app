// import
mod types;

// namespace
use clap::Parser;
use crate::types::cli::Cli;

fn main() {
  let args = Cli::parse();

  // args.path needs filename extension
  // correct: cargo run hello src/main.rs
  // incorrect: cargo run hello src/main
  let content = std::fs::read_to_string(&args.path).expect("the file is not found");
  for line in content.lines() {
    if line.contains(&args.pattern) {
      println!("{}", line);
    }
  }
}
