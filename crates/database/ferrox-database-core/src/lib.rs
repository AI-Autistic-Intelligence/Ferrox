//! # Ferrox Database Core (`ferrox-database-core`)
//!
//! `ferrox-database-core` defines the abstract persistence layer for Ferrox applications, introducing the generic `Repository<T, Id>` trait.
//!
//! ## Inversion of Control & Persistence Decoupling
//! By depending on `Repository<T, Id>` traits rather than concrete database drivers (SQL, Mongo, Redis), domain logic and services stay
//! completely decoupled from database engine details. This enables seamless unit testing via in-memory mock repositories.
//!
//! ## Key Features
//! - 📥 **Generic `Repository<Entity, Id>` Trait**: Standard `find_by_id`, `find_all`, `save`, `update`, and `delete` methods.
//! - 🧪 **Testing Mocks**: Simplify unit tests without launching database containers.

use async_trait::async_trait;
use ferrox_errors::AppError;

/// Core Repository Trait
/// This is the equivalent of the base repository in TypeORM/NestJS.
/// Implementations (like SeaORM or Mongo) will implement this trait for specific entities.
#[async_trait]
pub trait Repository<Entity, Id> {
    /// Finds a single entity by its primary key
    async fn find_by_id(&self, id: Id) -> Result<Option<Entity>, AppError>;
    
    /// Finds all entities
    async fn find_all(&self) -> Result<Vec<Entity>, AppError>;
    
    /// Inserts a new entity
    async fn insert(&self, entity: Entity) -> Result<Entity, AppError>;
    
    /// Updates an existing entity
    async fn update(&self, id: Id, entity: Entity) -> Result<Entity, AppError>;
    
    /// Deletes an entity by its primary key
    async fn delete(&self, id: Id) -> Result<(), AppError>;
}

pub fn setup() {
    println!("ferrox-database-core initialized: Provides Repository traits.");
}