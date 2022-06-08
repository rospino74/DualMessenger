#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod commands;
mod adb;

fn main() {
  tauri::Builder::default()
    .invoke_handler(commands::enumerate_native_handlers())
    .invoke_handler(adb::enumerate_adb_handlers())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}