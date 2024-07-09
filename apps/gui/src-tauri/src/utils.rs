use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
pub struct ValidationSettings {
  allow_files: bool,
  allow_directories: bool,
  allow_nonexistent: bool,
}

#[tauri::command]
pub fn validate_path(path: String, settings: ValidationSettings) -> bool {
  let rust_path = Path::new(&path);

  // If we don't allow nonexistent and the path doesn't exist, return false
  if !settings.allow_nonexistent && !rust_path.exists() {
    return false;
  }

  // If we don't allow files and we've got a file selected
  if !settings.allow_files && rust_path.is_file() {
    return false;
  }
  // If we don't allow directories and we've got a directory selected
  if !settings.allow_directories && rust_path.is_dir() {
    return false;
  }

  // Return true if all of the above were passed
  true
}

#[cfg(test)]
mod tests {
  use std::fs;

  use tempfile::TempDir;

  use super::*;

  #[test]
  fn test_validate_path() {
    let temp_dir = TempDir::new().unwrap();
    let test_dir_path = temp_dir
      .path()
      .join("test_dir")
      .to_str()
      .unwrap()
      .to_string();

    // Test non-existent temporary directory (not allowed)
    // ///////////////////////////////////////////////////
    let settings = ValidationSettings {
      allow_files: false,
      allow_directories: true,
      allow_nonexistent: false,
    };
    // Should not be valid
    assert!(!validate_path(test_dir_path.clone(), settings));

    // Test non-existent temporary directory (allowed)
    // ///////////////////////////////////////////////
    let settings = ValidationSettings {
      allow_files: false,
      allow_directories: true,
      allow_nonexistent: true,
    };
    // Should be valid
    assert!(validate_path(test_dir_path.clone(), settings));

    // Test directory exists but not allowed
    // /////////////////////////////////////
    fs::create_dir_all(&test_dir_path).unwrap(); // Create the directory
    let settings = ValidationSettings {
      allow_files: false,
      allow_directories: false,
      allow_nonexistent: false,
    };
    // Should not be valid
    assert!(!validate_path(test_dir_path.clone(), settings));

    // Test file not allowed
    // /////////////////////
    let test_file_path = Path::new(&test_dir_path)
      .join("test_file.txt")
      .to_string_lossy()
      .to_string();
    fs::write(&test_file_path, "Hello, World!").expect("Unable to write to file");
    let settings = ValidationSettings {
      allow_files: false,
      allow_directories: false,
      allow_nonexistent: false,
    };
    // Should not be valid
    assert!(!validate_path(test_file_path.clone(), settings));

    // Test file allowed
    // /////////////////
    let settings = ValidationSettings {
      allow_files: true,
      allow_directories: false,
      allow_nonexistent: false,
    };
    // Should be valid
    assert!(validate_path(test_file_path.clone(), settings));
  }

  #[test]
  fn test_deserialize_validation_settings() {
    let validation_settings_json = r#"{
      "allow_files": true,
      "allow_directories": false,
      "allow_nonexistent": true
    }"#;

    let validation_settings: ValidationSettings =
      serde_json::from_str(validation_settings_json).expect("Could not deserialize JSON");

    assert_eq!(
      validation_settings,
      ValidationSettings {
        allow_files: true,
        allow_directories: false,
        allow_nonexistent: true
      }
    )
  }
}
