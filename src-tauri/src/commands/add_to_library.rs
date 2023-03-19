use futures::{stream, StreamExt};
use num::ToPrimitive;
use enclose::enclose;
use std::path::{Path, PathBuf};
use tauri::async_runtime::spawn_blocking;
use tokio::fs;
use uuid::Uuid;

use crate::{metadata::ImageMetadata, repository::{RepositoryError, Repository}, AppState};

const FILE_PATTERN: &'static str = "*.{jpg,png,arw,raf,dng,tif}";
const CONCURRENT_ADDING: usize = 4;

async fn get_image_metadata(path: PathBuf) -> anyhow::Result<ImageMetadata> {
  let file_data = fs::metadata(&path).await?;

  println!("opening {}", path.display());

  spawn_blocking(enclose!((path) move || {
    let meta = rexiv2::Metadata::new_from_path(&path)?;

    Ok(ImageMetadata {
      full_path: path,
      aperture: meta.get_fnumber(),
      focal_length: meta.get_focal_length(),
      shot_at: file_data.created()?.into(),
      camera_name: meta.get_tag_string("Exif.Image.Model").ok(),
      iso: meta.get_iso_speed(),
      equivalent_focal_length: Some(meta.get_tag_numeric("Exif.Photo.FocalLengthIn35mmFilm")),
      lens_model: meta.get_tag_string("Exif.Photo.LensModel").ok(),
      exposure_time: meta.get_exposure_time().and_then(|n| n.to_f64()),
    })
  }))
  .await?
}

async fn add_file_to_library<P: AsRef<Path>>(repo: Repository, path: P) -> Result<Uuid, RepositoryError> {
  let data = get_image_metadata(PathBuf::from(path.as_ref())).await?;
  repo.insert_or_update_photo(data).await
}

#[tauri::command]
pub async fn add_to_library(state: tauri::State<'_, AppState>, root_path: String, recursive: bool) -> Result<(), RepositoryError> {
  println!("starting to go through \"{}\"", root_path);
  let path = Path::new(&root_path);
  if !path.is_dir() {
    return Err(anyhow::format_err!("Is not a directory: {}", root_path).into());
  }

  let file_paths: Vec<PathBuf> =
    globwalk::GlobWalkerBuilder::from_patterns(root_path, &[FILE_PATTERN])
      .follow_links(false)
      .max_depth(if recursive { 5 } else { 1 })
      .case_insensitive(true)
      .build()
      .map_err(|e| anyhow::format_err!("{}", e.to_string()))?
      .filter_map(Result::ok)
      .map(|img| img.into_path())
      .collect();

  stream::iter(file_paths)
    .map(|p| add_file_to_library(state.repo.clone(), p))
    .buffer_unordered(CONCURRENT_ADDING)
    .for_each(|res| async {
      match res {
        Ok(id) => println!("added file id {}", id),
        Err(e) => eprintln!("failed to add file: {}", e),
      }
    })
    .await;

  Ok(())
}
