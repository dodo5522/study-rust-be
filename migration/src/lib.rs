pub use sea_orm_migration::prelude::*;

mod m20251101_000001_create_table_sites;
mod m20251101_000002_create_table_cells;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251101_000001_create_table_sites::Migration),
            Box::new(m20251101_000002_create_table_cells::Migration),
        ]
    }
}
