// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let builder = tauri::Builder::default();

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
