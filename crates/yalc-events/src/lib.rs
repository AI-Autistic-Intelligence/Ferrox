use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tokio::sync::broadcast;
use yalc_errors::AppError;

/// A marker trait for all domain events.
pub trait DomainEvent: Clone + Send + Sync + Debug + Serialize + for<'de> Deserialize<'de> {
    fn event_name(&self) -> &'static str;
}

/// Abstract Event Dispatcher (CQRS/PubSub pattern)
#[async_trait]
pub trait EventDispatcher<E: DomainEvent>: Send + Sync {
    async fn publish(&self, event: E) -> Result<(), AppError>;
    async fn subscribe(&self) -> Result<broadcast::Receiver<E>, AppError>;
}

/// An In-Memory implementation of the EventDispatcher using Tokio broadcast channels.
/// Fast, zero-network overhead, perfect for intra-process event driven architectures.
pub struct InMemoryDispatcher<E: DomainEvent> {
    sender: broadcast::Sender<E>,
}

impl<E: DomainEvent> InMemoryDispatcher<E> {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }
}

#[async_trait]
impl<E: DomainEvent> EventDispatcher<E> for InMemoryDispatcher<E> {
    async fn publish(&self, event: E) -> Result<(), AppError> {
        // It's okay if there are no receivers, it just returns an error which we can ignore
        let _ = self.sender.send(event);
        Ok(())
    }

    async fn subscribe(&self) -> Result<broadcast::Receiver<E>, AppError> {
        Ok(self.sender.subscribe())
    }
}

pub fn setup() {
    println!("yalc-events initialized: Provides InMemory CQRS Event Dispatcher.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    // 1. TDD: Define a strongly typed domain event
    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
    struct UserCreatedEvent {
        user_id: u32,
        email: String,
    }

    impl DomainEvent for UserCreatedEvent {
        fn event_name(&self) -> &'static str {
            "UserCreated"
        }
    }

    // 2. TDD: Test the dispatcher logic
    #[tokio::test]
    async fn test_in_memory_dispatcher_pub_sub() {
        let dispatcher = InMemoryDispatcher::<UserCreatedEvent>::new(10);
        
        // Subscribe first
        let mut rx1 = dispatcher.subscribe().await.unwrap();
        let mut rx2 = dispatcher.subscribe().await.unwrap();

        // Publish event
        let event = UserCreatedEvent {
            user_id: 101,
            email: "test@yalc.com".into(),
        };
        dispatcher.publish(event.clone()).await.unwrap();

        // Verify subscribers receive the exact type-safe event
        let received1 = rx1.recv().await.unwrap();
        let received2 = rx2.recv().await.unwrap();

        assert_eq!(received1, event);
        assert_eq!(received2, event);
    }
}
