use std::{fs::File, path::PathBuf};

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
    "7z" | "zip" | "tar" | "gz" | "xz" | "rar" => {
      let archive_file = File::open(path)?;
      compress_tools::uncompress_archive(archive_file, out_path, compress_tools::Ownership::Ignore)
        .map_err(|e| e.into())
    }
    _ => Err(RZipError::UnsupportedArchive(ext.to_string())),
  }
}
