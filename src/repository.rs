use crate::models::Order;

pub async fn create_order(order: &Order) -> Result<(), String> {
    // Simulate database insertion (replace with your actual DB logic)
    println!("Simulating saving order to database: {:?}", order);
    Ok(())
}