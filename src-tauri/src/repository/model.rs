use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
  pub exposure_time: Option<f64>,
  pub preview_image: Option<Vec<u8>>,
}
