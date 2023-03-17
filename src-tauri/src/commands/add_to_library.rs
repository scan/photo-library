use globwalk::glob;
use num::ToPrimitive;
use std::{
  fs,
  path::{Path, PathBuf},
};

use crate::repository::{Photo, RepositoryError};

#[derive(Debug, Clone, Default)]
struct ImageMetadata {
  full_path: String,
  shot_at: chrono::DateTime<chrono::Utc>,
  camera_name: Option<String>,
  aperture: Option<f64>,
  focal_length: Option<f64>,
  equivalent_focal_length: Option<i32>,
  iso: Option<i32>,
  lens_model: Option<String>,
  exposure_time: Option<f64>,
}

async fn get_image_metadata(path: &str) -> anyhow::Result<ImageMetadata> {
  let meta = rexiv2::Metadata::new_from_path(path)?;
  let file_data = fs::metadata(path)?;

  // println!("{:?}", meta.get_tag_string("Exif.Image.DateTime"));

  Ok(ImageMetadata {
    full_path: path.to_owned(),
    aperture: meta.get_fnumber(),
    focal_length: meta.get_focal_length(),
    shot_at: file_data.created()?.into(),
    camera_name: meta.get_tag_string("Exif.Image.Model").ok(),
    iso: meta.get_iso_speed(),
    equivalent_focal_length: Some(meta.get_tag_numeric("Exif.Photo.FocalLengthIn35mmFilm")),
    lens_model: meta.get_tag_string("Exif.Photo.LensModel").ok(),
    exposure_time: meta.get_exposure_time().and_then(|n| n.to_f64()),
  })
}

impl ImageMetadata {
  fn as_photo_data(&self) -> anyhow::Result<Photo> {
    let path = Path::new(&self.full_path);
    if !path.exists() || path.is_dir() {
      anyhow::bail!("photo file does not exist: {}", path.to_string_lossy());
    }
    let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
    let file_type = path.extension().unwrap().to_string_lossy().into_owned();

    Ok(Photo {
      id: uuid::Uuid::new_v4(),
      full_path: self.full_path.clone(),
      file_name,
      file_type,
      shot_at: self.shot_at,
      camera_name: self.camera_name.clone(),
      aperture: self.aperture,
      focal_length: self.focal_length,
      equivalent_focal_length: self.equivalent_focal_length.map(|n| n as f64),
      iso: self.iso.map(|n| n as u32),
      lens_model: self.lens_model.clone(),
      exposure_time: self.exposure_time,
      preview_image: None,
    })
  }
}

#[tauri::command]
pub async fn add_to_library(root_path: &str, recursive: bool) -> Result<(), RepositoryError> {
  println!("starting to go through \"{}\"", root_path);
  let path = Path::new(root_path);
  if !path.is_dir() {
    return Err(anyhow::format_err!("Is not a directory: {}", root_path).into());
  }

  let pattern = "*.{jpg,png,arw,raf,dng,tif}";

  let file_paths: Vec<PathBuf> = globwalk::GlobWalkerBuilder::from_patterns(root_path, &[pattern])
    .follow_links(false)
    .max_depth(if recursive { 5 } else { 1 })
    .case_insensitive(true)
    .build()
    .map_err(|e| anyhow::format_err!("{}", e.to_string()))?
    .filter_map(Result::ok)
    .map(|img| img.into_path())
    .collect();

  println!("Found files: {:?}", file_paths);

  Ok(())
}
