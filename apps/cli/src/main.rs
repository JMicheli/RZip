use std::path::PathBuf;

use clap::{ArgAction, Parser};

use rzip_lib::{self, RZipError, RZipExtractConfig};

/// Represents the parameters passed to the RZip utility when run from the command line.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct RZipParams {
  /// The path to search for files to unzip
  pub target_path: PathBuf,

  /// Do a live run (default: false)
  #[arg(long, action = ArgAction::SetTrue)]
  pub live: bool,

  /// The directory to output to
  #[arg(long)]
  pub out_dir: Option<PathBuf>,
}

impl From<RZipParams> for RZipExtractConfig {
  fn from(value: RZipParams) -> Self {
    Self {
      target_path: value.target_path,
      out_dir: value.out_dir,
    }
  }
}

fn main() {
  let params = RZipParams::parse();

  if !params.live {
    println!("Performing a dry-run. Set --live flag to execute unzip operation.");
  }

  // The target path needs to exist
  if !params.target_path.exists() {
    // A non-existent path is an error
    println!(
      "Target path {} doesn't exist, exiting.",
      params.target_path.display()
    );
    return;
  }
  // Branch based on path type
  if params.target_path.is_file() {
    match handle_file(params) {
      Ok(_) => (),
      Err(e) => println!("Single file extract encountered error: {e}"),
    }
  } else {
    match handle_dir(params) {
      Ok(_) => (),
      Err(e) => println!("Directory extract encountered error: {e}"),
    }
  }
}

/// The branch of the main execution sequence that handles an input
/// path pointing to a directory.
///
/// Works by detecting each archive in the directory and then unzipping
/// them as indicated by other parameters.
fn handle_dir(params: RZipParams) -> Result<(), RZipError> {
  // Get a list of zip archives at the target path
  let archives = rzip_lib::get_archives_in_dir(&params.target_path)?;

  // If no archives are found, exit early
  if archives.is_empty() {
    println!("Found no archives, exiting");
    return Ok(());
  }

  // Live/dry run headers
  if params.live {
    println!("Extracting {} archives...", archives.len());
  } else {
    println!("Dry run operations (archive => output path):");
  }

  // Perform extraction
  let is_live = params.live;
  let extract_config = params.into();
  for item_path in archives {
    let out_path = rzip_lib::get_out_path_for_archive(&item_path, &extract_config)?;
    if is_live {
      print!("{:?}... ", item_path);
      // Live run logic
      match rzip_lib::recursive_file_extract(&item_path, &out_path, &extract_config) {
        Ok(_) => print!("Done.\n"),
        Err(e) => print!("Error: {e}\n"),
      }
    } else {
      // Dry run (explains what it would have done)
      println!("{} => {}", item_path.display(), out_path.display());
    }
  }

  Ok(())
}

/// The branch of the main execution sequence that handles an input
/// path pointing to a file.
///
/// Works by ensuring that the file is an archive and then recursively
/// unzipping items within it.
fn handle_file(params: RZipParams) -> Result<(), RZipError> {
  if !rzip_lib::is_archive_filetype(&params.target_path) {
    return Err(RZipError::RuntimeError(format!(
      "Error: {} is not an archive",
      params.target_path.display()
    )));
  }

  let is_live = params.live;
  let extract_config: RZipExtractConfig = params.into();
  let out_path = rzip_lib::get_out_path_for_archive(&extract_config.target_path, &extract_config)?;
  if is_live {
    // Live run
    match rzip_lib::recursive_file_extract(&extract_config.target_path, &out_path, &extract_config)
    {
      Ok(_) => println!("Successfully extracted archive"),
      Err(e) => println!("Error extracting archive: {e}"),
    }
  } else {
    // Dry run
    println!("Dry run operations (archive => output path):");
    println!(
      "{} => {}",
      extract_config.target_path.display(),
      out_path.display()
    );
  }

  Ok(())
}

#[cfg(test)]
mod test {
  use std::{fs, path::Path};

  use tempfile::TempDir;

  use super::*;

  #[test]
  fn test_handle_dir() {
    let temp_dir = TempDir::new().unwrap();
    let target_path = temp_dir.path().join("test_data");
    fs::create_dir_all(&target_path).unwrap();
    let out_path = temp_dir.path().join("output/path/");
    copy_tar_gz_data_to(&target_path);

    // Run test function (dry run)
    let params = RZipParams {
      target_path: target_path.clone(),
      live: false,
      out_dir: Some(out_path.clone()),
    };
    handle_dir(params).unwrap();

    // Run test function (live run)
    let params = RZipParams {
      target_path: target_path.clone(),
      live: true,
      out_dir: Some(out_path.clone()),
    };
    handle_dir(params).unwrap();

    // Test expected files
    let packed_tar_gz_tar_gz = temp_dir.path().join("test_data/packed_tar_gz.tar.gz");
    let packed_tar_dir = out_path.join("packed_tar_gz.tar");
    let doc_tar_gz = out_path.join("packed_tar_gz.tar/doc_tar_gz.txt");
    assert!(packed_tar_gz_tar_gz.exists());
    assert!(packed_tar_dir.exists());
    assert!(doc_tar_gz.exists());
  }

  #[test]
  fn test_handle_file() {
    let temp_dir = TempDir::new().unwrap();
    let out_path = temp_dir.path().join("output/path/");
    copy_tar_gz_data_to(temp_dir.path());

    // Run test function (dry run)
    let params = RZipParams {
      target_path: temp_dir.path().join("packed_tar_gz.tar.gz"),
      live: false,
      out_dir: Some(out_path.clone()),
    };
    handle_file(params).unwrap();

    // Run test function (live run)
    let params = RZipParams {
      target_path: temp_dir.path().join("packed_tar_gz.tar.gz"),
      live: true,
      out_dir: Some(out_path.clone()),
    };
    handle_file(params).unwrap();

    // Test expected files
    let packed_tar_gz_tar_gz = temp_dir.path().join("packed_tar_gz.tar.gz");
    let packed_tar_dir = out_path.join("packed_tar_gz.tar");
    let doc_tar_gz = out_path.join("packed_tar_gz.tar/doc_tar_gz.txt");
    assert!(packed_tar_gz_tar_gz.exists());
    assert!(packed_tar_dir.exists());
    assert!(doc_tar_gz.exists());
  }

  fn get_individual_data_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../lib/tests/data/indiv")
  }

  /// Copies data to the input `temp_dir`. The data has the structure:
  ///
  /// ```bash
  /// packed_tar_gz.tar.gz
  /// └── doc_tar_gz.txt
  /// ```
  fn copy_tar_gz_data_to(temp_dir: &Path) {
    let data_root = get_individual_data_root();
    let packed_tar = data_root.join("packed_tar_gz.tar.gz");

    // Copy each item to temporary directory
    fs::copy(packed_tar, temp_dir.join("packed_tar_gz.tar.gz")).unwrap();
  }
}
