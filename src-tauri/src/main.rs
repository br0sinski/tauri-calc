// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;

fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![
    greet,
    utils::add,
    utils::sub,
    utils::div,
    utils::mult,
    utils::square
    ])
  .run(tauri::generate_context!())
  .expect("error launching tauri application!");
}

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}