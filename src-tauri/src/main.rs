// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;
mod services;


fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_positioner::init())
    .invoke_handler(tauri::generate_handler![
      services::note::get_memo_by_day,
      services::note::set_memo
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
