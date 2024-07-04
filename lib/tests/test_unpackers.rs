use std::{
  fs,
  path::{Path, PathBuf},
};

use tempfile::TempDir;

fn get_individual_data_root() -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/indiv")
}

/// Copies data to the input `temp_dir`. The data has the structure:
///
/// ```bash
/// packed_zip.zip
/// └── doc_zip.txt
/// ```
fn copy_zip_data_to(temp_dir: &Path) {
  let data_root = get_individual_data_root();
  let packed_zip = data_root.join("packed_zip.zip");

  // Copy each item to temporary directory
  fs::copy(packed_zip, temp_dir.join("packed_zip.zip")).unwrap();
}

#[test]
fn test_compress_tools_unpack() {
  let temp_dir = TempDir::new().unwrap();
  copy_zip_data_to(temp_dir.path());
  let target_path = temp_dir.path().join("packed_zip.zip");

  let config = rzip_lib::RZipExtractConfig {
    target_path: target_path.clone(),
    out_dir: None,
    delete_after_extracting: false,
  };

  let out_path = rzip_lib::get_out_path_for_archive(&config.target_path, &config).unwrap();
  rzip_lib::unpack::compress_tools_unpack(&target_path, &out_path).unwrap();

  // Test expected files
  let packed_zip_zip = temp_dir.path().join("packed_zip.zip");
  let packed_zip_dir = temp_dir.path().join("packed_zip");
  let doc_zip = temp_dir.path().join("packed_zip/doc_zip.txt");
  assert!(packed_zip_zip.exists());
  assert!(packed_zip_dir.exists());
  assert!(doc_zip.exists());
}

/// Copies data to the input `temp_dir`. The data has the structure:
///
/// ```bash
/// packed_7z.7z
/// └── doc_7z.txt
/// ```
fn copy_7z_data_to(temp_dir: &Path) {
  let data_root = get_individual_data_root();
  let packed_7z = data_root.join("packed_7z.7z");

  // Copy each item to temporary directory
  fs::copy(packed_7z, temp_dir.join("packed_7z.7z")).unwrap();
}

#[test]
fn test_seven_z_unpack() {
  let temp_dir = TempDir::new().unwrap();
  copy_7z_data_to(temp_dir.path());
  let target_path = temp_dir.path().join("packed_7z.7z");

  let config = rzip_lib::RZipExtractConfig {
    target_path: target_path.clone(),
    out_dir: None,
    delete_after_extracting: false,
  };

  let out_path = rzip_lib::get_out_path_for_archive(&config.target_path, &config).unwrap();
  rzip_lib::unpack::seven_z_unpack(&target_path, &out_path).unwrap();

  // Test expected files
  let packed_7z_7z = temp_dir.path().join("packed_7z.7z");
  let packed_7z_dir = temp_dir.path().join("packed_7z");
  let doc_7z = temp_dir.path().join("packed_7z/doc_7z.txt");
  assert!(packed_7z_7z.exists());
  assert!(packed_7z_dir.exists());
  assert!(doc_7z.exists());
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

#[test]
fn test_flake2_unpack() {
  let temp_dir = TempDir::new().unwrap();
  copy_tar_gz_data_to(temp_dir.path());
  let target_path = temp_dir.path().join("packed_tar_gz.tar.gz");

  let config = rzip_lib::RZipExtractConfig {
    target_path: target_path.clone(),
    out_dir: None,
    delete_after_extracting: false,
  };

  let out_path = rzip_lib::get_out_path_for_archive(&config.target_path, &config).unwrap();
  rzip_lib::unpack::flake2_unpack(&target_path, &out_path).unwrap();

  // Test expected files
  let packed_tar_gz_tar_gz = temp_dir.path().join("packed_tar_gz.tar.gz");
  let packed_tar_dir = temp_dir.path().join("packed_tar_gz.tar");
  let doc_tar_gz = temp_dir.path().join("packed_tar_gz.tar/doc_tar_gz.txt");
  assert!(packed_tar_gz_tar_gz.exists());
  assert!(packed_tar_dir.exists());
  assert!(doc_tar_gz.exists());
}

/// Copies data to the input `temp_dir`. The data has the structure:
///
/// ```bash
/// packed_tar.tar
/// └── doc_tar.txt
/// ```
fn copy_tar_data_to(temp_dir: &Path) {
  let data_root = get_individual_data_root();
  let packed_tar = data_root.join("packed_tar.tar");

  // Copy each item to temporary directory
  fs::copy(packed_tar, temp_dir.join("packed_tar.tar")).unwrap();
}

#[test]
fn test_tar_unpack() {
  let temp_dir = TempDir::new().unwrap();
  copy_tar_data_to(temp_dir.path());
  let target_path = temp_dir.path().join("packed_tar.tar");

  let config = rzip_lib::RZipExtractConfig {
    target_path: target_path.clone(),
    out_dir: None,
    delete_after_extracting: false,
  };

  let out_path = rzip_lib::get_out_path_for_archive(&config.target_path, &config).unwrap();
  rzip_lib::unpack::tar_unpack(&target_path, &out_path).unwrap();

  // Test expected files
  let packed_tar_tar = temp_dir.path().join("packed_tar.tar");
  let packed_tar_dir = temp_dir.path().join("packed_tar");
  let doc_tar = temp_dir.path().join("packed_tar/doc_tar.txt");
  assert!(packed_tar_tar.exists());
  assert!(packed_tar_dir.exists());
  assert!(doc_tar.exists());
}
