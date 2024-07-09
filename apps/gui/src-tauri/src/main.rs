// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, fs, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TreeDataNode {
  title: String,
  key: String,
  icon: Option<String>,
  children: Option<Vec<TreeDataNode>>,
}

#[tauri::command]
fn get_file_tree(path: String) -> Vec<TreeDataNode> {
  let root_path = Path::new(&path);

  vec![build_tree_node(root_path, "0")]
}

#[derive(Deserialize, Debug)]
pub struct ValidationSettings {
  allow_files: bool,
  allow_directories: bool,
  allow_nonexistent: bool,
}

#[tauri::command]
fn validate_path(path: String, settings: ValidationSettings) -> bool {
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

fn build_tree_node(path: &Path, key_prefix: &str) -> TreeDataNode {
  let title = path
    .file_name()
    .unwrap_or(path.as_os_str())
    .to_string_lossy()
    .into_owned();

  let mut children = Vec::new();
  let mut idx = 0;

  if path.is_dir() {
    if let Ok(entries) = fs::read_dir(path) {
      for entry in entries.filter_map(Result::ok) {
        let child_key = format!("{}-{}", key_prefix, idx);
        children.push(build_tree_node(&entry.path(), &child_key));
        idx += 1;
      }
    }
  }

  TreeDataNode {
    title,
    key: key_prefix.to_string(),
    icon: Some(determine_icon(path).to_string()),
    children: if children.is_empty() {
      None
    } else {
      Some(children)
    },
  }
}

fn determine_icon(path: &Path) -> &'static str {
  if path.is_dir() {
    "FolderOutlined"
  } else {
    "FileOutlined"
  }
}

#[tauri::command]
fn start_extraction() {
  println!("start_extraction() running");
}

#[tauri::command]
fn do_refresh() {
  println!("do_refresh() running");
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      start_extraction,
      validate_path,
      do_refresh,
      get_file_tree
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
