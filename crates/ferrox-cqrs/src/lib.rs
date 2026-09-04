use async_trait::async_trait;
use ferrox_errors::AppError;

/// A Command represents an intent to change the system's state.
pub trait Command: Send + Sync {}

/// A Query represents a request for information without changing state.
pub trait Query: Send + Sync {}

#[async_trait]
pub trait CommandHandler<C: Command, R> {
    async fn execute(&self, command: C) -> Result<R, AppError>;
}

#[async_trait]
pub trait QueryHandler<Q: Query, R> {
    async fn fetch(&self, query: Q) -> Result<R, AppError>;
}

/// The CommandBus routes commands to their appropriate handlers.
/// In a real implementation, this would use a registry of handlers or Dependency Injection.
pub struct CommandBus;

impl CommandBus {
    pub async fn execute<C, R, H>(&self, handler: &H, command: C) -> Result<R, AppError>
    where
        C: Command,
        H: CommandHandler<C, R>,
    {
        handler.execute(command).await
    }
}

pub fn setup() {
    println!("ferrox-cqrs initialized: CommandBus and QueryBus are ready for DDD architectures.");
}
