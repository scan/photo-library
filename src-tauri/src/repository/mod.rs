mod error;

use std::fs;

use chrono::{DateTime, Utc};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tauri::{api::path::app_local_data_dir, Config};
use sqlx::migrate::Migrator;
use uuid::Uuid;

pub use self::error::RepositoryError;

static MIGRATOR: Migrator = sqlx::migrate!();


pub struct Photo {
  id: Uuid,
  full_path: String,
  file_name: String,
  file_type: String,
  shot_at: DateTime<Utc>,
  camera_name: Option<String>,
  aperture: Option<f64>,
  focal_length: Option<f64>,
  equivalent_focal_length: Option<f64>,
  iso: Option<u32>,
  lens_model: Option<String>,
  preview_image: Option<Vec<u8>>
}

pub struct Repository {
  pool: SqlitePool
}

fn get_database_url(config: &Config) -> Result<String, RepositoryError> {
  let data_dir = app_local_data_dir(config).unwrap_or(".".into());
  fs::create_dir_all(data_dir.clone())?;

  let db_path = data_dir.join("library.db".to_owned());

  Ok(format!("sqlite://{}", db_path.display().to_string()))
}

pub async fn open_repository(config: &Config) -> Result<Repository, RepositoryError> {
  let url = get_database_url(config)?;

  if !Sqlite::database_exists(&url).await.unwrap_or(false) {
    println!("Creating database {}", url);
    Sqlite::create_database(&url).await?;
  } else {
    println!("Database already exists");
  }

  let db = SqlitePool::connect(&url).await?;
  MIGRATOR.run(&db).await?;

  Ok(Repository { pool: db })
}
