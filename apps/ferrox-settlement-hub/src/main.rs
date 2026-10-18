//! ferrox-settlement-hub Entry Point
use ferrox_settlement_hub::application::{CreateOrderCommand, OrderApplicationService};

#[tokio::main]
async fn main() {
    println!("🚀 Launching Ferrox Clean Service: ferrox-settlement-hub");
    let order = OrderApplicationService::handle_create(CreateOrderCommand {
        customer_id: "demo_customer".to_string(),
        quantity: 10,
    }).expect("Order creation failed");
    println!("✅ Order created with ID: {}", order.id());
}
