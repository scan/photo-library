use std::path::PathBuf;
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct ImageMetadata {
  pub full_path: PathBuf,
  pub shot_at: chrono::DateTime<chrono::Utc>,
  pub camera_name: Option<String>,
  pub aperture: Option<f64>,
  pub focal_length: Option<f64>,
  pub equivalent_focal_length: Option<i32>,
  pub iso: Option<i32>,
  pub lens_model: Option<String>,
  pub exposure_time: Option<f64>,
}
