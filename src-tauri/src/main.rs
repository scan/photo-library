// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

use repository::open_repository;

mod repository;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
struct ImageMetadata {
  #[serde(rename = "fileName")]
  file_name: String,
  #[serde(rename = "fullPath")]
  full_path: String,
  #[serde(rename = "shotAt")]
  shot_at: chrono::DateTime<chrono::Utc>,
  #[serde(rename = "cameraName")]
  camera_name: Option<String>,
  aperture: Option<f64>,
  #[serde(rename = "focalLength")]
  focal_length: Option<f64>,
  #[serde(rename = "equivalentFocalLength")]
  equivalent_focal_length: Option<i32>,
  iso: Option<i32>,
  #[serde(rename = "lensModel")]
  lens_model: Option<String>,
}

#[tauri::command]
async fn get_image_metadata(paths: Vec<String>) -> Result<Vec<ImageMetadata>, String> {
  println!("paths given: {:?}", paths);

  paths
    .iter()
    .map(|path| {
      let meta = rexiv2::Metadata::new_from_path(&path).map_err(|err| err.to_string())?;

      let file_data = fs::metadata(path).map_err(|e| e.to_string())?;

      println!("{:?}", meta.get_tag_string("Exif.Image.DateTime"));

      Ok(ImageMetadata {
        full_path: path.clone(),
        aperture: meta.get_fnumber(),
        focal_length: meta.get_focal_length(),
        shot_at: file_data.created().map_err(|e| e.to_string())?.into(),
        camera_name: meta.get_tag_string("Exif.Image.Model").ok(),
        iso: meta.get_iso_speed(),
        equivalent_focal_length: Some(meta.get_tag_numeric("Exif.Photo.FocalLengthIn35mmFilm")),
        lens_model: meta.get_tag_string("Exif.Photo.LensModel").ok(),
        ..Default::default()
      })
    })
    .into_iter()
    .collect()
}

#[tokio::main]
async fn main() {
  tauri::async_runtime::set(tokio::runtime::Handle::current());
  rexiv2::initialize().expect("error initiatilizing exiv2");

  let context = tauri::generate_context!();

  open_repository(context.config()).await;

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_image_metadata])
    .run(context)
    .expect("error while running tauri application");
}
