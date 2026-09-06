//! # Ferrox Database SeaORM (`ferrox-database-seaorm`)
//!
//! `ferrox-database-seaorm` provides concrete implementation of `ferrox-database-core` traits for relational databases
//! (PostgreSQL, MySQL, SQLite) using the SeaORM async ORM engine.
//!
//! ## Key Features
//! - 🐘 **Multi-Database Support**: Connect to Postgres, MySQL, or SQLite seamlessly.
//! - 🔄 **Async Connection Pooling**: Managed connection pools with configurable pool limits and timeouts.
//! - 🛠️ **SeaORM Integration**: Native integration with SeaORM entities and active models.

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;
use ferrox_errors::AppError;

/// Wrapper around SeaORM DatabaseConnection
#[derive(Clone)]
pub struct SeaOrmClient {
    pub db: DatabaseConnection,
}

impl SeaOrmClient {
    /// Connects to the database and sets up connection pooling
    pub async fn connect(database_url: &str) -> Result<Self, AppError> {
        let mut opt = ConnectOptions::new(database_url.to_owned());
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);

        let db = Database::connect(opt)
            .await
            .map_err(|e: DbErr| AppError::DatabaseError(e.to_string()))?;

        Ok(Self { db })
    }
}

pub fn setup() {
    println!("ferrox-database-seaorm initialized: Provides SeaOrmClient connection pool.");
}