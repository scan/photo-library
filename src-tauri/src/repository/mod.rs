mod error;
mod migrator;
mod model;

use std::ops::Deref;
use tokio::fs;

use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::prelude::*;
use tauri::{api::path::app_local_data_dir, Config};

pub use self::error::RepositoryError;
pub use self::model::Photo;

use self::migrator::Migrator;

#[derive(Clone)]
pub struct Repository(DatabaseConnection);

impl Deref for Repository {
  type Target = DatabaseConnection;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Repository {
  pub async fn insert_photo(&self, photo: &Photo) -> Result<(), RepositoryError> {
    Ok(())
  }
}

async fn get_database_url(config: &Config) -> Result<String, RepositoryError> {
  let data_dir = app_local_data_dir(config).unwrap_or(".".into());
  if !fs::try_exists(&data_dir).await? {
    fs::create_dir_all(&data_dir).await?;
  }

  let db_path = data_dir.join("library.db".to_owned());

  Ok(format!(
    "sqlite://{}?mode=rwc",
    db_path.display().to_string()
  ))
}

pub async fn open_repository(config: &Config) -> Result<Repository, RepositoryError> {
  let url = get_database_url(config).await?;
  let db = Database::connect(url).await?;

  Migrator::refresh(&db).await?;

  Ok(Repository(db))
}
