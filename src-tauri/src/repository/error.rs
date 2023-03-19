use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
  #[error(transparent)]
  Database(#[from] sea_orm::DbErr),
  #[error(transparent)]
  DatabaseTransaction(#[from] sea_orm::TransactionError<sea_orm::DbErr>),
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Other(#[from] anyhow::Error)
}

impl Serialize for RepositoryError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}
