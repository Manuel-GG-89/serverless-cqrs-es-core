use serde_derive::{Deserialize, Serialize};
use order_core::entities::Order;
use order_core::command_types::{CreateOrder, ChangeOrderStatus};

// all the structs that are used in the command_handler

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AppsyncObjectResponse {
    // put here all the possible responses
    Order(Order),
    Error(String),
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum EnabledCommand {
    CreateOrder(CreateOrder),
    ChangeOrderStatus(ChangeOrderStatus),
}
