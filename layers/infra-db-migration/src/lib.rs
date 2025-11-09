pub use sea_orm_migration::prelude::*;

mod iden;
mod m20251101_000001_create_schemas;
mod m20251101_000002_create_table_sites;
mod m20251101_000003_create_table_devices;
mod m20251109_104531_create_junction_table_for_sites_and_devices;
mod m20251109_113902_add_set_updated_at_routine;
mod m20251109_125856_add_columns_of_created_at_and_updated_at;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251101_000001_create_schemas::Migration),
            Box::new(m20251101_000002_create_table_sites::Migration),
            Box::new(m20251101_000003_create_table_devices::Migration),
            Box::new(m20251109_104531_create_junction_table_for_sites_and_devices::Migration),
            Box::new(m20251109_113902_add_set_updated_at_routine::Migration),
            Box::new(m20251109_125856_add_columns_of_created_at_and_updated_at::Migration),
        ]
    }
}
