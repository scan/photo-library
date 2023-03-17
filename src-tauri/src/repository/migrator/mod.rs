use sea_orm_migration::prelude::*;

mod m20230313_074333_create_photos;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![Box::new(m20230313_074333_create_photos::Migration)]
  }
}
