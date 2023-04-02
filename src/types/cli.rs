#[derive(Debug)]
pub struct Cli {
  pub pattern: String,
  pub path: std::path::PathBuf,
}
