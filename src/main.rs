use std::{fs, path::PathBuf};

use clap::{ArgAction, Parser};

const ARCHIVE_EXTENSIONS: [&str; 5] = ["zip", "xz", "tar", "gz", "rar"];

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct RZipCli {
  /// The path to search for files to unzip
  target_path: PathBuf,

  /// Do a live run (default: false)
  #[arg(long, action = ArgAction::SetTrue)]
  live: bool,

  /// Should delete archives (default: false)
  #[arg(short, long, action = ArgAction::SetTrue)]
  delete_archives: bool,
}

fn main() {
  let cli = RZipCli::parse();

  println!("Value for target_path: {:?}", cli.target_path);
  if !cli.live {
    println!("Performing a dry-run. Set --live flag to execute unzip operation.");
  }

  // Get a list of zip archives at the target path
  let archives = match get_archive_entries(&cli.target_path) {
    Ok(val) => val,
    Err(e) => {
      println!("Encountered an error: {e}");
      return;
    }
  };

  if archives.is_empty() {
    println!("Found no archives");
    return;
  }

  println!("Found archives:");
  for item in archives {
    println!("{:?}", item)
  }
}

fn get_archive_entries(path: &PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
  let mut output_entries = Vec::new();

  let read_entries = fs::read_dir(path)?;
  for entry in read_entries {
    // Handle directory vs file
    let path = entry?.path();
    if path.is_dir() {
      // For directories, we recurse
      let subpath_entries = get_archive_entries(&path)?;
      output_entries.extend(subpath_entries);
    } else if is_archive_filetype(&path) {
      // Files get pushed onto the vector of archive entries
      output_entries.push(path);
    }
  }

  Ok(output_entries)
}

fn is_archive_filetype(path: &PathBuf) -> bool {
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
