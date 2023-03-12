CREATE TABLE IF NOT EXISTS photos (
  id TEXT NOT NULL PRIMARY KEY,
  full_path TEXT NOT NULL UNIQUE,
  file_name TEXT NOT NULL,
  file_type TEXT NOT NULL,
  shot_at TEXT NOT NULL,
  camera_name TEXT,
  aperture REAL,
  focal_length REAL,
  equivalent_focal_length REAL,
  iso INTEGER,
  lens_model TEXT,
  preview_image BLOB
);

CREATE INDEX idx_photos_shot_at ON photos (shot_at);
CREATE INDEX idx_photos_camera_name ON photos (camera_name, shot_at);
CREATE INDEX idx_photos_aperture ON photos (aperture, shot_at);
CREATE INDEX idx_photos_focal_length ON photos (focal_length, shot_at);
CREATE INDEX idx_photos_equivalent_focal_length ON photos (equivalent_focal_length, shot_at);
CREATE INDEX idx_photos_iso ON photos (iso, shot_at);
CREATE INDEX idx_photos_lens_model ON photos (lens_model, shot_at);
CREATE INDEX idx_photos_all ON photos (camera_name, aperture, focal_length, equivalent_focal_length, iso, lens_model);
