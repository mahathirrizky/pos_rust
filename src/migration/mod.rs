
pub use sea_orm_migration::prelude::*;

mod m20250918_024324_create_table;
mod m20250922_190000_create_refund_tables;
mod m20250922_210000_create_promotions;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250918_024324_create_table::Migration),
            Box::new(m20250922_190000_create_refund_tables::Migration),
            Box::new(m20250922_210000_create_promotions::Migration),
        ]
    }
}