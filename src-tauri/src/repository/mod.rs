mod entities;
mod error;
mod migrator;

use std::ops::Deref;
use tokio::fs;

use sea_orm::{prelude::*, ActiveValue, IntoActiveModel};
use sea_orm::{Database, DatabaseConnection, TransactionTrait};
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
          use ActiveValue::Set;

          let full_path: String = meta.full_path.to_string_lossy().into();

          let existing = photo::Entity::find()
            .filter(photo::Column::FullPath.eq(&full_path))
            .one(txn)
            .await?;

          match existing {
            Some(p) => {
              let id = p.id;
              let mut photo_model = p.into_active_model();

              photo_model.shot_at = Set(meta.shot_at);
              photo_model.camera_name = Set(meta.camera_name);
              photo_model.aperture = Set(meta.aperture);
              photo_model.focal_length = Set(meta.focal_length);
              photo_model.equvalent_focal_length =
                Set(meta.equivalent_focal_length.map(|n| n as f64));
              photo_model.iso = Set(meta.iso);
              photo_model.lens_model = Set(meta.lens_model);
              photo_model.exposure_time = Set(meta.exposure_time);

              photo_model.save(txn).await?;
              Ok(id)
            }
            None => {
              let id = Uuid::new_v4();

              let photo_model = photo::ActiveModel {
                id: Set(id),
                full_path: Set(full_path),
                file_name: Set(String::from(
                  meta
                    .full_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy(),
                )),
                file_extension: Set(
                  meta
                    .full_path
                    .extension()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_ascii_lowercase(),
                ),
                shot_at: Set(meta.shot_at),
                camera_name: Set(meta.camera_name),
                aperture: Set(meta.aperture),
                focal_length: Set(meta.focal_length),
                equvalent_focal_length: Set(meta.equivalent_focal_length.map(|n| n as f64)),
                iso: Set(meta.iso),
                lens_model: Set(meta.lens_model),
                exposure_time: Set(meta.exposure_time),
                ..Default::default()
              };

              photo_model.insert(txn).await?;

              Ok(id)
            }
          }
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
