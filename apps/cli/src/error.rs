use thiserror::Error;

#[derive(Error, Debug)]
pub enum RZipError {
  #[error("compress-tools error: {0}")]
  CompressTools(#[from] compress_tools::Error),
  #[error("IO error occurred: {0}")]
  Io(#[from] std::io::Error),
  #[error("Runtime error: {0}")]
  RuntimeError(String),
  #[error("Unsupported archive: {0}")]
  UnsupportedArchive(String),
}
