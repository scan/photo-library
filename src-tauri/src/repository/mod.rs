use std::fs;

use sqlx::{migrate::MigrateDatabase, Sqlite};
use tauri::{api::path::app_local_data_dir, Config};

fn get_database_url(config: &Config) -> String {
  let data_dir = app_local_data_dir(config).unwrap_or(".".into());
  fs::create_dir_all(data_dir.clone()).expect("error creating data directory");

  let db_path = data_dir.join("library.db".to_owned());

  format!("sqlite://{}", db_path.display().to_string())
}

pub async fn open_repository(config: &Config) {
  let url = get_database_url(config);

  if !Sqlite::database_exists(&url).await.unwrap_or(false) {
    println!("Creating database {}", url);
    match Sqlite::create_database(&url).await {
      Ok(_) => println!("Create db success"),
      Err(error) => panic!("error: {}", error),
    }
  } else {
    println!("Database already exists");
  }
}
