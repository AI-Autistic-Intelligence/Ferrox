//! Application Layer - CQRS Command Handlers

use crate::domain::{DomainError, OrderAggregate};

pub struct CreateOrderCommand {
    pub customer_id: String,
    pub quantity: u32,
}

pub struct OrderApplicationService;

impl OrderApplicationService {
    pub fn handle_create(cmd: CreateOrderCommand) -> Result<OrderAggregate, DomainError> {
        OrderAggregate::new(cmd.customer_id, cmd.quantity)
    }
}
