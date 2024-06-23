use std::{
  fs,
  path::{Path, PathBuf},
};

use tempfile::TempDir;

use rzip_lib;

/// Copies "series" data from test data to the `temp_dir` provided. The data has this structure:
///
/// ```bash
/// doc_set1.zip
/// ├── doc1.txt
/// ├── doc2.txt
/// └── doc3.txt
/// doc_set2.zip
/// ├── doc4.txt
/// ├── doc5.txt
/// └── doc6.txt
/// doc_set3.zip
/// ├── doc7.txt
/// ├── doc8.txt
/// └── doc9.txt
/// ```
fn copy_series_data_to(temp_dir: &Path) {
  let series_data_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/series");
  let doc_set1 = series_data_root.join("doc_set1.zip");
  let doc_set2 = series_data_root.join("doc_set2.zip");
  let doc_set3 = series_data_root.join("doc_set3.zip");

  // Copy each item to temporary directory
  fs::copy(doc_set1, temp_dir.join("doc_set1.zip")).unwrap();
  fs::copy(doc_set2, temp_dir.join("doc_set2.zip")).unwrap();
  fs::copy(doc_set3, temp_dir.join("doc_set3.zip")).unwrap();
}

#[test]
fn test_series_no_out_dir() {
  let temp_dir = TempDir::new().unwrap();
  let temp_dir_path_buf = temp_dir.path().to_path_buf();
  copy_series_data_to(temp_dir.path());

  let config = rzip_lib::RZipExtractConfig {
    target_path: temp_dir_path_buf.clone(),
    out_dir: None,
  };

  let archives = rzip_lib::get_archives_in_dir(&temp_dir_path_buf).unwrap();

  for archive in archives {
    let out_path = rzip_lib::get_out_path_for_archive(&archive, &config).unwrap();
    rzip_lib::recursive_file_extract(&archive, &out_path, &config).unwrap()
  }

  // Test the existence of each expected file
  let doc_set1_zip = temp_dir.path().join("doc_set1.zip");
  let doc_set1_dir = temp_dir.path().join("doc_set1");
  let doc1_txt = temp_dir.path().join("doc_set1/doc1.txt");
  let doc2_txt = temp_dir.path().join("doc_set1/doc2.txt");
  let doc3_txt = temp_dir.path().join("doc_set1/doc3.txt");
  let doc_set2_zip = temp_dir.path().join("doc_set2.zip");
  let doc_set2_dir = temp_dir.path().join("doc_set2");
  let doc4_txt = temp_dir.path().join("doc_set2/doc4.txt");
  let doc5_txt = temp_dir.path().join("doc_set2/doc5.txt");
  let doc6_txt = temp_dir.path().join("doc_set2/doc6.txt");
  let doc_set3_zip = temp_dir.path().join("doc_set3.zip");
  let doc_set3_dir = temp_dir.path().join("doc_set3");
  let doc7_txt = temp_dir.path().join("doc_set3/doc7.txt");
  let doc8_txt = temp_dir.path().join("doc_set3/doc8.txt");
  let doc9_txt = temp_dir.path().join("doc_set3/doc9.txt");
  // Assertions
  assert!(doc_set1_zip.exists());
  assert!(doc_set1_dir.exists());
  assert!(doc1_txt.exists());
  assert!(doc2_txt.exists());
  assert!(doc3_txt.exists());
  assert!(doc_set2_zip.exists());
  assert!(doc_set2_dir.exists());
  assert!(doc4_txt.exists());
  assert!(doc5_txt.exists());
  assert!(doc6_txt.exists());
  assert!(doc_set3_zip.exists());
  assert!(doc_set3_dir.exists());
  assert!(doc7_txt.exists());
  assert!(doc8_txt.exists());
  assert!(doc9_txt.exists());
}

#[test]
fn test_series_with_out_dir() {
  let temp_dir = TempDir::new().unwrap();
  let temp_dir_path_buf = temp_dir.path().to_path_buf();
  copy_series_data_to(temp_dir.path());

  let out_dir_path = temp_dir.path().join("out/dir/path/here");

  let config = rzip_lib::RZipExtractConfig {
    target_path: temp_dir_path_buf.clone(),
    out_dir: Some(out_dir_path.clone()),
  };

  let archives = rzip_lib::get_archives_in_dir(&temp_dir_path_buf).unwrap();

  for archive in archives {
    let out_path = rzip_lib::get_out_path_for_archive(&archive, &config).unwrap();
    rzip_lib::recursive_file_extract(&archive, &out_path, &config).unwrap()
  }

  // Test the existence of each expected file
  let doc_set1_dir = temp_dir.path().join("out/dir/path/here/doc_set1");
  let doc1_txt = temp_dir.path().join("out/dir/path/here/doc_set1/doc1.txt");
  let doc2_txt = temp_dir.path().join("out/dir/path/here/doc_set1/doc2.txt");
  let doc3_txt = temp_dir.path().join("out/dir/path/here/doc_set1/doc3.txt");
  let doc_set2_dir = temp_dir.path().join("out/dir/path/here/doc_set2");
  let doc4_txt = temp_dir.path().join("out/dir/path/here/doc_set2/doc4.txt");
  let doc5_txt = temp_dir.path().join("out/dir/path/here/doc_set2/doc5.txt");
  let doc6_txt = temp_dir.path().join("out/dir/path/here/doc_set2/doc6.txt");
  let doc_set3_dir = temp_dir.path().join("out/dir/path/here/doc_set3");
  let doc7_txt = temp_dir.path().join("out/dir/path/here/doc_set3/doc7.txt");
  let doc8_txt = temp_dir.path().join("out/dir/path/here/doc_set3/doc8.txt");
  let doc9_txt = temp_dir.path().join("out/dir/path/here/doc_set3/doc9.txt");
  // Assertions
  assert!(doc_set1_dir.exists());
  assert!(doc1_txt.exists());
  assert!(doc2_txt.exists());
  assert!(doc3_txt.exists());
  assert!(doc_set2_dir.exists());
  assert!(doc4_txt.exists());
  assert!(doc5_txt.exists());
  assert!(doc6_txt.exists());
  assert!(doc_set3_dir.exists());
  assert!(doc7_txt.exists());
  assert!(doc8_txt.exists());
  assert!(doc9_txt.exists());
}
