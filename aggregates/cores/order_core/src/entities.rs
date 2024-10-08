use serde_derive::{Deserialize, Serialize};
use std::fmt;

// Crea una estructura para manejar el objeto Order
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub id: String,
    pub event_number: i32,
    pub user_id: String,
    pub product_id: String,
    pub quantity: i32,
    pub created_at: String,
    pub order_status: OrderStatus,
}

// Implementa el trait Default para Order
impl Default for Order {
    fn default() -> Self {
        Order {
            id: String::from(""),
            event_number: 0,
            user_id: String::from(""),
            product_id: String::from(""),
            quantity: 0,
            created_at: String::from(""),
            order_status: OrderStatus::Pending,
        }
    }
}

// Implementa Display para la estructura Order
impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Order ID: {}\nLast Event Number: {}\nUser ID: {}\nProduct ID: {}\nQuantity: {}\nOrder Status: {:?}",
            self.id, self.event_number, self.user_id, self.product_id, self.quantity, self.order_status
        )
    }
}

// crea un enum  que reprecente los estados de una orden
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum OrderStatus {
    Pending,
    Created,
    Paid,
    Shipped,
    Delivered,
    Cancelled,
}

// Implementa Display para el enum OrderStatus
impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderStatus::Pending => write!(f, "Pending"),
            OrderStatus::Created => write!(f, "Created"),
            OrderStatus::Paid => write!(f, "Paid"),
            OrderStatus::Shipped => write!(f, "Shipped"),
            OrderStatus::Delivered => write!(f, "Delivered"),
            OrderStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}
