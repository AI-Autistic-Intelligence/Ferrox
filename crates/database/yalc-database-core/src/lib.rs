use async_trait::async_trait;
use yalc_errors::AppError;

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
    println!("yalc-database-core initialized: Provides Repository traits.");
}
