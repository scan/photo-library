mod entities;
mod error;
mod migrator;

use std::ops::Deref;
use tokio::fs;

use sea_orm::{prelude::*, ActiveValue};
use sea_orm::{Database, DatabaseConnection, TransactionTrait, TryIntoModel};
use sea_orm_migration::prelude::*;
use tauri::{api::path::app_local_data_dir, Config};
use uuid::Uuid;

use crate::metadata::ImageMetadata;

pub use self::entities::{prelude::*, *};
pub use self::error::RepositoryError;

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
  pub async fn insert_or_update_photo(&self, meta: ImageMetadata) -> Result<Uuid, RepositoryError> {
    let id = self
      .0
      .transaction::<_, Uuid, DbErr>(|txn| {
        Box::pin(async move {
          let full_path: String = meta.full_path.to_string_lossy().into();

          let existing = photo::Entity::find()
            .filter(photo::Column::FullPath.eq(&full_path))
            .one(txn)
            .await?;
          let photo_model = existing
            .map(photo::ActiveModel::from)
            .unwrap_or(photo::ActiveModel {
              id: ActiveValue::Set(Uuid::new_v4()),
              full_path: ActiveValue::Set(full_path.to_ascii_lowercase()),
              file_name: ActiveValue::Set(String::from(
                meta
                  .full_path
                  .file_name()
                  .unwrap_or_default()
                  .to_string_lossy()
                  .to_ascii_lowercase(),
              )),
              file_extension: ActiveValue::Set(
                meta
                  .full_path
                  .extension()
                  .unwrap_or_default()
                  .to_string_lossy()
                  .to_ascii_lowercase(),
              ),
              ..Default::default()
            });

          let photo_model = photo_model.save(txn).await?;
          let photo_model = photo_model.try_into_model()?;

          Ok(photo_model.id)
        })
      })
      .await?;

    Ok(id)
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
