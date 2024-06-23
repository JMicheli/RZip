mod error;
mod unpack;

use std::{fs, path::PathBuf};

pub use error::RZipError;

/// The list of extensinsions used to check if a file is an archive.
const ARCHIVE_EXTENSIONS: [&str; 6] = ["zip", "xz", "tar", "gz", "7z", "rar"];

pub struct RZipExtractConfig {
  pub target_path: PathBuf,
  pub out_dir: Option<PathBuf>,
}

pub fn recursive_file_extract(
  path: &PathBuf,
  out_path: &PathBuf,
  config: &RZipExtractConfig,
) -> Result<(), RZipError> {
  // Unpack the file
  unpack::unpack_file(path, out_path)?;

  // Check to see if there are any other zips after extraction and re-call self
  // on each if there are.
  let residual_archives = get_archives_in_dir(out_path)?;
  for res_path in residual_archives {
    let res_out_path = get_out_path_for_archive(&res_path, config)?;
    recursive_file_extract(&res_path, &res_out_path, config)?;
  }

  Ok(())
}

pub fn get_out_path_for_archive(
  archive_path: &PathBuf,
  config: &RZipExtractConfig,
) -> Result<PathBuf, RZipError> {
  let output_path = archive_path.file_stem().ok_or_else(|| {
    RZipError::RuntimeError(format!(
      "Unable to determine file stem for {}",
      archive_path.display()
    ))
  })?;

  if let Some(out_dir) = &config.out_dir {
    if config.target_path == *archive_path {
      return Ok(out_dir.join(output_path));
    }

    let relative_path = get_relative_path(archive_path, config, out_dir)?;
    Ok(construct_output_path(&relative_path, out_dir, output_path))
  } else {
    Ok(archive_path.parent().unwrap().join(output_path))
  }
}

fn get_relative_path(
  archive_path: &PathBuf,
  config: &RZipExtractConfig,
  out_dir: &PathBuf,
) -> Result<PathBuf, RZipError> {
  let res = if archive_path.starts_with(&config.target_path) {
    archive_path.strip_prefix(&config.target_path)
  } else {
    archive_path.strip_prefix(&out_dir)
  };

  res
    .map_err(|e| RZipError::RuntimeError(format!("Strip prefix error: {}", e)))
    .map(|res| res.to_path_buf())
}

fn construct_output_path(
  relative_path: &PathBuf,
  out_dir: &PathBuf,
  output_path: &std::ffi::OsStr,
) -> PathBuf {
  let rel_dir = relative_path.parent().unwrap(); // handle errors appropriately
  out_dir.join(rel_dir).join(output_path)
}

pub fn is_archive_filetype(path: &PathBuf) -> bool {
  // We only collect files
  if path.is_dir() {
    return false;
  }

  // If we have an extension
  if let Some(ext) = path.extension() {
    // And it has a str representation present in the extensions array
    if let Some(ext_str) = ext.to_str() {
      return ARCHIVE_EXTENSIONS.contains(&ext_str);
    }
  }

  false
}

pub fn get_archives_in_dir(path: &PathBuf) -> Result<Vec<PathBuf>, RZipError> {
  let mut output_entries = Vec::new();

  let read_entries = fs::read_dir(path)?;
  for entry in read_entries {
    // Handle directory vs file
    let path = entry?.path();
    if path.is_dir() {
      // For directories, we recurse
      let subpath_entries = get_archives_in_dir(&path)?;
      output_entries.extend(subpath_entries);
    } else if is_archive_filetype(&path) {
      // Files get pushed onto the vector of archive entries
      output_entries.push(path);
    }
  }

  Ok(output_entries)
}
