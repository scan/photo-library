// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use repository::open_repository;

mod repository;
mod commands;

#[tokio::main]
async fn main() {
  tauri::async_runtime::set(tokio::runtime::Handle::current());
  rexiv2::initialize().expect("error initiatilizing exiv2");

  let context = tauri::generate_context!();

  open_repository(context.config()).await.expect("error opening database");

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![commands::add_to_library])
    .run(context)
    .expect("error while running tauri application");
}
