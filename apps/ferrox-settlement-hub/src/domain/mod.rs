//! Pure Domain Aggregate Root - Zero ORM or HTTP Framework Dependencies

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("Quantity must be strictly positive")]
    InvalidQuantity,
    #[error("Cannot cancel an order that is already completed")]
    AlreadyCompleted,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Approved,
    Cancelled,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderAggregate {
    id: Uuid,
    customer_id: String,
    quantity: u32,
    status: OrderStatus,
}

impl OrderAggregate {
    pub fn new(customer_id: impl Into<String>, quantity: u32) -> Result<Self, DomainError> {
        if quantity == 0 {
            return Err(DomainError::InvalidQuantity);
        }
        Ok(Self {
            id: Uuid::new_v4(),
            customer_id: customer_id.into(),
            quantity,
            status: OrderStatus::Pending,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn status(&self) -> &OrderStatus {
        &self.status
    }

    pub fn approve(&mut self) -> Result<(), DomainError> {
        if self.status == OrderStatus::Cancelled {
            return Err(DomainError::AlreadyCompleted);
        }
        self.status = OrderStatus::Approved;
        Ok(())
    }

    pub fn cancel(&mut self) -> Result<(), DomainError> {
        if self.status == OrderStatus::Approved {
            return Err(DomainError::AlreadyCompleted);
        }
        self.status = OrderStatus::Cancelled;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_creation_invariant() {
        let order = OrderAggregate::new("cust_123", 5).unwrap();
        assert_eq!(*order.status(), OrderStatus::Pending);
        assert!(OrderAggregate::new("cust_123", 0).is_err());
    }

    #[test]
    fn test_order_state_transitions() {
        let mut order = OrderAggregate::new("cust_456", 2).unwrap();
        assert!(order.approve().is_ok());
        assert_eq!(*order.status(), OrderStatus::Approved);
        assert!(order.cancel().is_err());
    }
}
