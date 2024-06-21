use std::path::PathBuf;

use crate::error::RZipError;

pub fn unpack_file(path: &PathBuf, out_path: &PathBuf) -> Result<(), RZipError> {
  // Get extension
  // TODO - Handle .tar.gz
  let ext = path
    .extension()
    .ok_or(RZipError::RuntimeError(format!(
      "Failed to get extension for {}",
      path.display()
    )))?
    .to_str()
    .ok_or(RZipError::RuntimeError(format!(
      "Failed to convert extension to str: {}",
      path.display()
    )))?;

  match ext {
    "7z" => sevenz_rust::decompress_file(path, out_path).map_err(|e| e.into()),
    _ => Err(RZipError::UnsupportedArchive(ext.to_string())),
  }
}
