use serde_derive::{Deserialize, Serialize};
use super::entities::OrderStatus;
use std::fmt;



// Crea un comando para manejar la creación de una orden
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrder {
    pub id: String,
    pub user_id: String,
    pub product_id: String,
    pub quantity: i32,
}


impl fmt::Display for CreateOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CreateOrder {{ id: {}, user_id: {}, product_id: {}, quantity: {} }}",
            self.id, self.user_id, self.product_id, self.quantity
        )
    }
}


// Crea un comando para cambiar el estado de una order
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangeOrderStatus {
    pub id: String,
    pub last_event_number: i32,
    pub order_status: OrderStatus,
}               


impl fmt::Display for ChangeOrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ChangeOrderStatus {{ id: {}, last_event_number: {}, order_status: {:?} }}",
            self.id, self.last_event_number, self.order_status
        )
    }
}