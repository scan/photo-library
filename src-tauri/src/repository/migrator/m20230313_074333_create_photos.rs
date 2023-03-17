use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Photo::Table)
          .if_not_exists()
          .col(ColumnDef::new(Photo::Id).uuid().not_null().primary_key())
          .col(
            ColumnDef::new(Photo::FullPath)
              .string()
              .not_null()
              .unique_key(),
          )
          .col(ColumnDef::new(Photo::FileName).string().not_null())
          .col(ColumnDef::new(Photo::FileExtension).string().not_null())
          .col(ColumnDef::new(Photo::ShotAt).date_time().not_null())
          .col(ColumnDef::new(Photo::CameraName).string())
          .col(ColumnDef::new(Photo::Aperture).decimal())
          .col(ColumnDef::new(Photo::FocalLength).decimal())
          .col(ColumnDef::new(Photo::EquvalentFocalLength).decimal())
          .col(ColumnDef::new(Photo::Iso).integer())
          .col(ColumnDef::new(Photo::LensModel).string())
          .col(ColumnDef::new(Photo::ExposureTime).decimal())
          .col(ColumnDef::new(Photo::PreviewImage).blob(BlobSize::Long))
          .to_owned(),
      )
      .await?;

    manager
      .create_index(
        Index::create()
          .name("idx_photos_shot_at")
          .if_not_exists()
          .table(Photo::Table)
          .col(Photo::ShotAt)
          .to_owned(),
      )
      .await?;

    manager
      .create_index(
        Index::create()
          .name("idx_photos_camera_name")
          .if_not_exists()
          .table(Photo::Table)
          .col(Photo::CameraName)
          .col(Photo::ShotAt)
          .to_owned(),
      )
      .await?;

    manager
      .create_index(
        Index::create()
          .name("idx_photos_aperture")
          .if_not_exists()
          .table(Photo::Table)
          .col(Photo::Aperture)
          .col(Photo::ShotAt)
          .to_owned(),
      )
      .await?;

    manager
      .create_index(
        Index::create()
          .name("idx_photos_focal_length")
          .if_not_exists()
          .table(Photo::Table)
          .col(Photo::FocalLength)
          .col(Photo::ShotAt)
          .to_owned(),
      )
      .await?;

    manager
      .create_index(
        Index::create()
          .name("idx_photos_equivalent_focal_length")
          .if_not_exists()
          .table(Photo::Table)
          .col(Photo::EquvalentFocalLength)
          .col(Photo::ShotAt)
          .to_owned(),
      )
      .await?;

    manager
      .create_index(
        Index::create()
          .name("idx_photos_iso")
          .if_not_exists()
          .table(Photo::Table)
          .col(Photo::Iso)
          .col(Photo::ShotAt)
          .to_owned(),
      )
      .await?;

    manager
      .create_index(
        Index::create()
          .name("idx_photos_lens_model")
          .if_not_exists()
          .table(Photo::Table)
          .col(Photo::LensModel)
          .col(Photo::ShotAt)
          .to_owned(),
      )
      .await?;

    manager
      .create_index(
        Index::create()
          .name("idx_exposure_time")
          .if_not_exists()
          .table(Photo::Table)
          .col(Photo::ExposureTime)
          .col(Photo::ShotAt)
          .to_owned(),
      )
      .await?;

    manager
      .create_index(
        Index::create()
          .name("idx_photos_all")
          .if_not_exists()
          .table(Photo::Table)
          .col(Photo::CameraName)
          .col(Photo::Aperture)
          .col(Photo::FocalLength)
          .col(Photo::EquvalentFocalLength)
          .col(Photo::Iso)
          .col(Photo::LensModel)
          .col(Photo::ExposureTime)
          .col(Photo::ShotAt)
          .to_owned(),
      )
      .await?;

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Photo::Table).to_owned())
      .await
  }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Photo {
  Table,
  Id,
  FullPath,
  FileName,
  FileExtension,
  ShotAt,
  CameraName,
  Aperture,
  FocalLength,
  EquvalentFocalLength,
  Iso,
  LensModel,
  ExposureTime,
  PreviewImage,
}
