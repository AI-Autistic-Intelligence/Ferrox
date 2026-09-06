//! # Ferrox Migrations (`ferrox-migrations`)
//!
//! `ferrox-migrations` provides programmatic execution of database schema migrations for Ferrox applications.
//!
//! ## Key Features
//! - 🚀 **Automated Migration Runner**: Executes pending migrations automatically at service boot time.
//! - 📜 **SeaORM & SQL Support**: Works with SeaORM migrations or raw `.sql` script directories.

use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // Box::new(m20220101_000001_create_table::Migration),
        ]
    }
}

pub async fn run_migrations(db: &sea_orm::DatabaseConnection) -> Result<(), DbErr> {
    Migrator::up(db, None).await?;
    tracing::info!("Database schema migrations applied successfully.");
    Ok(())
}