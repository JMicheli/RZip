// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
    .invoke_handler(tauri::generate_handler![start_extraction, do_refresh])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
