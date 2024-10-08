
use aws_lambda_events::event::eventbridge::EventBridgeEvent;
use aws_config::load_from_env; 
use aws_sdk_dynamodb::Client;
use shared_library::convert_json_value_to_hashmap;
use lambda_runtime::{tracing, Error, LambdaEvent};



pub async fn event_handler(event: LambdaEvent<EventBridgeEvent>) -> Result<(), Error>  {
    
    let shared_config = load_from_env().await; // Update this line to use the imported `load_from_env` function
    let client = Client::new(&shared_config);
    
    // Extract some useful info from the request
    let event_detail = &event.payload.detail;

    let json_event = match convert_json_value_to_hashmap(event_detail.clone()) {
        Ok(event) => {
            tracing::info!("Evento recibido desde EventBridge: {:?}", event);
            event
        },
        Err(e) => {
            tracing::error!("ERROR al serializar el evento recibido desde EventBridge {:?}", e);
            return Err(Error::from(format!("ERROR al serializar el evento recibido desde EventBridge {:?}", e)))
        },
    };

    let request = client
        .put_item()
        .table_name("order-eventstore")
        .set_item(Some(json_event));

    let response = request.send().await?;

    tracing::info!("Evento guardado correctamente. Dynamodb Response: {:?}", response);

    Ok(())
}