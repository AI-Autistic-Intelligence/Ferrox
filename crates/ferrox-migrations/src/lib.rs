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
