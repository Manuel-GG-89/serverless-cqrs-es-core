use serde_derive::{Deserialize, Serialize};
use super::entities::OrderStatus;

// Crea un evento OrderCreated
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderCreated {
    pub event_number: i32,
    pub event_name: String,
    pub id: String,
    pub user_id: String,
    pub product_id: String,
    pub quantity: i32,
    pub created_at: String,
    pub order_status: OrderStatus,
}

// Crea un evento OrderStatusChanged
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderStatusChanged {
    pub event_number: i32,
    pub event_name: String,
    pub id: String,
    pub order_status: OrderStatus,
}


