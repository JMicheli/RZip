use std::{fs::File, path::PathBuf};

use crate::error::{RZipError, RZipProcessingError};

type UnpackStage = fn(&PathBuf, &PathBuf) -> Result<(), RZipProcessingError>;

pub fn unpack_file(path: &PathBuf, out_path: &PathBuf) -> Result<RZipError, RZipError> {
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

  let unpack_stages: Vec<UnpackStage> = match ext {
    "zip" => vec![compress_tools_unpack],
    "7z" => vec![compress_tools_unpack, seven_z_unpack],
    "gz" | "tgz" => vec![compress_tools_unpack],
    "xz" => vec![compress_tools_unpack],
    "tar" => vec![compress_tools_unpack],
    "rar" => vec![compress_tools_unpack],
    _ => {
      return Err(RZipError::RuntimeError(format!(
        "Didn't recognize extension {}",
        ext
      )))
    }
  };

  let mut errors = vec![];
  for stage_fn in unpack_stages {
    let res = stage_fn(path, out_path);
    match res {
      Ok(_) => return Ok(RZipError::ProcessingErrors(errors)),
      Err(e) => {
        errors.push(e);
      }
    }
  }

  // If we reach the very end instead of a stage returning Ok then we have failed.
  Err(RZipError::ProcessingErrors(errors))
}

pub fn compress_tools_unpack(
  archive_path: &PathBuf,
  out_path: &PathBuf,
) -> Result<(), RZipProcessingError> {
  let archive_file: File = File::open(archive_path)?;
  compress_tools::uncompress_archive(archive_file, out_path, compress_tools::Ownership::Ignore)
    .map_err(|e| e.into())
}

pub fn seven_z_unpack(
  archive_path: &PathBuf,
  out_path: &PathBuf,
) -> Result<(), RZipProcessingError> {
  sevenz_rust::decompress_file(archive_path, out_path).map_err(|e| e.into())
}
