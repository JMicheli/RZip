use std::{
  fs,
  path::{Path, PathBuf},
};

use tempfile::TempDir;

use rzip_lib;

/// Copies "nested" data from test data to the `temp_dir` provided. The data has this structure:
///
/// ```bash
/// ├── doc_set1.zip
/// │   ├── doc1.txt
/// │   ├── doc2.txt
/// │   └── doc3.txt
/// ├── doc_set2.zip
/// │   ├── doc4.txt
/// │   ├── doc5.txt
/// │   └── doc6.txt
/// ├── doc_set3.zip
/// │   ├── doc7.txt
/// │   ├── doc8.txt
/// │   └── doc9.txt
/// ├── doc1.txt
/// ├── doc2.txt
/// └── doc3.txt
/// ```
fn copy_nested_data_to(temp_dir: &Path) {
  let series_data_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/nested");
  let nested = series_data_root.join("nested.zip");

  // Copy each item to temporary directory
  fs::copy(nested, temp_dir.join("nested.zip")).unwrap();
}

#[test]
fn test_nested_no_out_dir() {
  let temp_dir = TempDir::new().unwrap();
  let target_path = temp_dir.path().join("nested.zip");
  copy_nested_data_to(temp_dir.path());

  let config = rzip_lib::RZipExtractConfig {
    target_path: target_path.clone(),
    out_dir: None,
  };

  let out_path = rzip_lib::get_out_path_for_archive(&target_path, &config).unwrap();
  rzip_lib::recursive_file_extract(&target_path, &out_path, &config).unwrap();

  // Test the existence of each expected file
  let nested = temp_dir.path().join("nested");
  let doc_set1_zip = nested.join("doc_set1.zip");
  let doc_set1 = nested.join("doc_set1");
  let doc_set1_doc1 = nested.join("doc_set1/doc1.txt");
  let doc_set1_doc2 = nested.join("doc_set1/doc2.txt");
  let doc_set1_doc3 = nested.join("doc_set1/doc3.txt");
  let doc_set2_zip = nested.join("doc_set2.zip");
  let doc_set2 = nested.join("doc_set2");
  let doc_set2_doc4 = nested.join("doc_set2/doc4.txt");
  let doc_set2_doc5 = nested.join("doc_set2/doc5.txt");
  let doc_set2_doc6 = nested.join("doc_set2/doc6.txt");
  let doc_set3_zip = nested.join("doc_set3.zip");
  let doc_set3 = nested.join("doc_set3");
  let doc_set3_doc7 = nested.join("doc_set3/doc7.txt");
  let doc_set3_doc8 = nested.join("doc_set3/doc8.txt");
  let doc_set3_doc9 = nested.join("doc_set3/doc9.txt");
  let doc1 = nested.join("doc1.txt");
  let doc2 = nested.join("doc2.txt");
  let doc3 = nested.join("doc3.txt");
  // Do assertions
  assert!(nested.exists());
  assert!(doc_set1_zip.exists());
  assert!(doc_set1.exists());
  assert!(doc_set1_doc1.exists());
  assert!(doc_set1_doc2.exists());
  assert!(doc_set1_doc3.exists());
  assert!(doc_set2_zip.exists());
  assert!(doc_set2.exists());
  assert!(doc_set2_doc4.exists());
  assert!(doc_set2_doc5.exists());
  assert!(doc_set2_doc6.exists());
  assert!(doc_set3_zip.exists());
  assert!(doc_set3.exists());
  assert!(doc_set3_doc7.exists());
  assert!(doc_set3_doc8.exists());
  assert!(doc_set3_doc9.exists());
  assert!(doc1.exists());
  assert!(doc2.exists());
  assert!(doc3.exists());
}

#[test]
fn test_nested_with_out_dir() {
  let temp_dir = TempDir::new().unwrap();
  let target_path = temp_dir.path().join("nested.zip");
  copy_nested_data_to(temp_dir.path());

  let out_dir_buf = temp_dir.path().join("an/out/dir");

  let config = rzip_lib::RZipExtractConfig {
    target_path: target_path.clone(),
    out_dir: Some(out_dir_buf.clone()),
  };

  let out_path = rzip_lib::get_out_path_for_archive(&target_path, &config).unwrap();
  rzip_lib::recursive_file_extract(&target_path, &out_path, &config).unwrap();

  // Test the existence of each expected file
  let nested = temp_dir.path().join("an/out/dir/nested");
  let doc_set1_zip = nested.join("doc_set1.zip");
  let doc_set1 = nested.join("doc_set1");
  let doc_set1_doc1 = nested.join("doc_set1/doc1.txt");
  let doc_set1_doc2 = nested.join("doc_set1/doc2.txt");
  let doc_set1_doc3 = nested.join("doc_set1/doc3.txt");
  let doc_set2_zip = nested.join("doc_set2.zip");
  let doc_set2 = nested.join("doc_set2");
  let doc_set2_doc4 = nested.join("doc_set2/doc4.txt");
  let doc_set2_doc5 = nested.join("doc_set2/doc5.txt");
  let doc_set2_doc6 = nested.join("doc_set2/doc6.txt");
  let doc_set3_zip = nested.join("doc_set3.zip");
  let doc_set3 = nested.join("doc_set3");
  let doc_set3_doc7 = nested.join("doc_set3/doc7.txt");
  let doc_set3_doc8 = nested.join("doc_set3/doc8.txt");
  let doc_set3_doc9 = nested.join("doc_set3/doc9.txt");
  let doc1 = nested.join("doc1.txt");
  let doc2 = nested.join("doc2.txt");
  let doc3 = nested.join("doc3.txt");
  // Do assertions
  assert!(nested.exists());
  assert!(doc_set1_zip.exists());
  assert!(doc_set1.exists());
  assert!(doc_set1_doc1.exists());
  assert!(doc_set1_doc2.exists());
  assert!(doc_set1_doc3.exists());
  assert!(doc_set2_zip.exists());
  assert!(doc_set2.exists());
  assert!(doc_set2_doc4.exists());
  assert!(doc_set2_doc5.exists());
  assert!(doc_set2_doc6.exists());
  assert!(doc_set3_zip.exists());
  assert!(doc_set3.exists());
  assert!(doc_set3_doc7.exists());
  assert!(doc_set3_doc8.exists());
  assert!(doc_set3_doc9.exists());
  assert!(doc1.exists());
  assert!(doc2.exists());
  assert!(doc3.exists());
}
