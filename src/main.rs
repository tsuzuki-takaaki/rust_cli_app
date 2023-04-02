// import
mod types;

// namespace
use std::env::args;

fn main() {
  let pattern = args().nth(1).unwrap();
  let path = args().nth(2).unwrap();
  println!("pattern: {}, path: {}", pattern, path);
}
