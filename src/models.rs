use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    #[serde(default)]
    pub id: Option<Uuid>,
    pub customer_id: String,
    pub items: Vec<String>,
    pub total: f64,
}