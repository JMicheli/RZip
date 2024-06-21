mod common;
mod error;

use clap::Parser;

use common::RZipParams;
use error::RZipError;

fn main() {
  let params = RZipParams::parse();

  println!("Value for target_path: {:?}", params.target_path);
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
  let archives = common::get_archives_in_dir(&params.target_path)?;

  // If no archives are found, exit early
  if archives.is_empty() {
    println!("Found no archives, exiting");
    return Ok(());
  }

  println!("Extracting {} archives...", archives.len());
  for item_path in archives {
    print!("{:?}... ", item_path);

    let out_path = common::get_out_path_for_archive(&item_path, &params)?;
    match common::recursive_file_extract(&item_path, &out_path, &params) {
      Ok(_) => print!("Done.\n"),
      Err(e) => print!("Error: {e}\n"),
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
  if !common::is_archive_filetype(&params.target_path) {
    return Err(RZipError::RuntimeError(format!(
      "Error: {} is not an archive",
      params.target_path.display()
    )));
  }

  // Extract
  let out_path = common::get_out_path_for_archive(&params.target_path, &params)?;
  match common::recursive_file_extract(&params.target_path, &out_path, &params) {
    Ok(_) => println!("Successfully extracted archive"),
    Err(e) => println!("Error extracting archive: {e}"),
  }

  Ok(())
}
