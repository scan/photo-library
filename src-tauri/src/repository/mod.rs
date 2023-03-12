mod error;

use std::fs;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::migrate::Migrator;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tauri::{api::path::app_local_data_dir, Config};
use uuid::Uuid;

pub use self::error::RepositoryError;

static MIGRATOR: Migrator = sqlx::migrate!();

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Photo {
  pub id: Uuid,
  pub full_path: String,
  pub file_name: String,
  pub file_type: String,
  pub shot_at: DateTime<Utc>,
  pub camera_name: Option<String>,
  pub aperture: Option<f64>,
  pub focal_length: Option<f64>,
  pub equivalent_focal_length: Option<f64>,
  pub iso: Option<u32>,
  pub lens_model: Option<String>,
  pub preview_image: Option<Vec<u8>>,
}

pub struct Repository {
  pool: SqlitePool,
}

impl Repository {
  pub async fn insert_photo(&self, photo: &Photo) -> Result<(), RepositoryError> {
    sqlx::query!(
      r#"INSERT INTO photos (id, full_path, file_name, file_type, shot_at, camera_name, aperture, focal_length, equivalent_focal_length, iso, lens_model) VALUES (?,?,?,?,?,?,?,?,?,?,?)"#,
      photo.id, photo.full_path, photo.file_name, photo.file_type, photo.shot_at, photo.camera_name, photo.aperture, photo.focal_length, photo.equivalent_focal_length, photo.iso, photo.lens_model
    ).execute(&self.pool).await?;

    Ok(())
  }
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
