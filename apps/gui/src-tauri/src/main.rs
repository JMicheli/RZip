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
fn get_file_tree() -> Vec<TreeDataNode> {
  let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
  let root_path = Path::new(&manifest_dir);

  vec![build_tree_node(root_path, "0")]
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
      do_refresh,
      get_file_tree
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
