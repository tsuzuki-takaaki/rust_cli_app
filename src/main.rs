// import
mod types;

// namespace
use clap::Parser;
use crate::types::cli::Cli;

fn main() {
  let args = Cli::parse();
  println!("{:?}", args);
}
