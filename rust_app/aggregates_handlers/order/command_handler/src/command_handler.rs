use aggregates_common_lib::appsync_event_structure::AppsyncEvent;
use lambda_runtime::tracing;
use lambda_runtime::{Error, LambdaEvent};
use super::commands::*;
use super::commands_processor::*;


pub async fn command_handler(event: LambdaEvent<AppsyncEvent>) -> Result<AppsyncObjectResponse, Error> {

    tracing::info!("Event received from AppSync");
    // Extract some useful info from the request
    let command_name: &str = &event.payload.info.field_name;
    tracing::info!("Received command: {}", command_name);   

    // Prepare the response
    let appsync_respose = match command_name {
        "createOrder" => create_order(&event.payload).await,
        "changeOrderStatus" => change_order_status(&event.payload).await,
        _ => Err(Error::from("Command not found")),
    };

    tracing::info!("COMANDO PROCESADO. Response: {:?}", appsync_respose);

    return appsync_respose
}