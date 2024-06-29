use thiserror::Error;

#[derive(Error, Debug)]
pub enum RZipError {
  #[error("Extration error: {0:?}")]
  ProcessingErrors(Vec<RZipProcessingError>),
  #[error("IO error occurred: {0}")]
  Io(#[from] std::io::Error),
  #[error("Runtime error: {0}")]
  RuntimeError(String),
  #[error("Unsupported archive: {0}")]
  UnsupportedArchive(String),
}

#[derive(Error, Debug)]
pub enum RZipProcessingError {
  #[error("Compress-tools error: {0}")]
  CompressTools(#[from] compress_tools::Error),
  #[error("Seven-z error: {0}")]
  SevenZ(#[from] sevenz_rust::Error),
  #[error("IO error occurred: {0}")]
  Io(#[from] std::io::Error),
}
