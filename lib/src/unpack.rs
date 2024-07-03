use std::{fs::File, path::Path};

use crate::error::{RZipError, RZipProcessingError};

type UnpackStage = fn(&Path, &Path) -> Result<(), RZipProcessingError>;

pub fn unpack_file(path: &Path, out_path: &Path) -> Result<RZipError, RZipError> {
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
    "gz" | "tgz" => vec![compress_tools_unpack, flake2_unpack],
    "xz" => vec![compress_tools_unpack],
    "tar" => vec![compress_tools_unpack, tar_unpack],
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

/// Unpack an archive using the [compress_tools] backend.
///
/// Documentation: https://github.com/OSSystems/compress-tools-rs/.
pub fn compress_tools_unpack(
  archive_path: &Path,
  out_path: &Path,
) -> Result<(), RZipProcessingError> {
  let archive_file: File = File::open(archive_path)?;
  compress_tools::uncompress_archive(archive_file, out_path, compress_tools::Ownership::Ignore)
    .map_err(|e| e.into())
}

/// Unpack an archive using the [sevenz_rust] backend.
///
/// Documentation: https://github.com/dyz1990/sevenz-rust
pub fn seven_z_unpack(archive_path: &Path, out_path: &Path) -> Result<(), RZipProcessingError> {
  sevenz_rust::decompress_file(archive_path, out_path).map_err(|e| e.into())
}

/// Unpack an archive using the [flate2] backend.
///
/// Documentation: https://docs.rs/flate2/latest/flate2/
pub fn flake2_unpack(archive_path: &Path, out_path: &Path) -> Result<(), RZipProcessingError> {
  use flate2::read::GzDecoder;
  use tar::Archive;

  let tar_gz = File::open(archive_path)?;
  let tar = GzDecoder::new(tar_gz);
  let mut archive = Archive::new(tar);
  archive.unpack(out_path)?;

  Ok(())
}

/// Unpack an archive using the [tar] backend.
///
/// Documentation: https://docs.rs/tar/latest/tar/
pub fn tar_unpack(archive_path: &Path, out_path: &Path) -> Result<(), RZipProcessingError> {
  use tar::Archive;

  let tar = File::open(archive_path).unwrap();
  let mut archive: Archive<File> = Archive::new(tar);
  archive.unpack(out_path)?;

  Ok(())
}
