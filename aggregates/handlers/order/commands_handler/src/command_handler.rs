use shared_library::appsync_structs::AppsyncEvent;
use lambda_runtime::tracing;
use lambda_runtime::{Error, LambdaEvent};
use super::commands::*;
use super::commands_processor::*;


pub async fn command_handler(command: LambdaEvent<AppsyncEvent>) -> Result<AppsyncObjectResponse, Error> {

    tracing::info!("Command received from AppSync");

    let command_name: &str = &command.payload.info.field_name;

    tracing::info!("Received command: {}", command_name);   

    let appsync_respose = match command_name {
        "createOrder" => create_order(&command.payload).await,
        "changeOrderStatus" => change_order_status(&command.payload).await,
        _ => Err(Error::from("Command not found")),
    };

    tracing::info!("COMANDO PROCESADO. Response: {:?}", appsync_respose);

    return appsync_respose
}