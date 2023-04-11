use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
  pub pattern: String,
  pub path: Option<std::path::PathBuf>,
}
