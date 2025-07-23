use sea_orm_migration::prelude::*;

mod m20240101_000001_create_users_table;
mod m20240101_000002_create_products_table;
mod m20240101_000003_create_orders_table;
mod m20240101_000004_create_payments_table;
mod m20240101_000005_create_inventory_table;
mod m20240101_000006_create_notifications_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240101_000001_create_users_table::Migration),
            Box::new(m20240101_000002_create_products_table::Migration),
            Box::new(m20240101_000003_create_orders_table::Migration),
            Box::new(m20240101_000004_create_payments_table::Migration),
            Box::new(m20240101_000005_create_inventory_table::Migration),
            Box::new(m20240101_000006_create_notifications_table::Migration),
        ]
    }
} 