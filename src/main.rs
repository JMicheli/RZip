use std::{
  fs::{self, DirEntry},
  path::PathBuf,
};

use clap::{ArgAction, Parser};

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

  let target_path = cli.target_path;
  println!("Value for target_path: {:?}", target_path);

  if !cli.live {
    println!("Performing a dry-run. Set --live flag to execute unzip operation.");
  }

  // Continued program logic goes here...
}

fn get_entries(path: &PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
  let mut output_entries = Vec::new();

  let read_entries = fs::read_dir(path)?;
  for entry in read_entries {
    let path = entry?.path();
    // Handle directory vs file
    if path.is_dir() {
      let subpath_entries = get_entries(&path)?;
      output_entries.extend(subpath_entries);
    } else {
      output_entries.push(path);
    }
  }

  Ok(output_entries)
}

const ARCHIVE_FILENAMES: [&str; 5] = ["zip", "xz", "tar", "gz", "rar"];

fn should_add_entry(path: &PathBuf) -> bool {
  // We only collect files
  if path.is_dir() {
    return false;
  }

  todo!()
}
