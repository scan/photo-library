#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
  #[error(transparent)]
  SqlError(#[from] sqlx::Error),
  #[error(transparent)]
  MigrateError(#[from] sqlx::migrate::MigrateError),
  #[error(transparent)]
  Io(#[from] std::io::Error),
}

impl serde::Serialize for RepositoryError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}
