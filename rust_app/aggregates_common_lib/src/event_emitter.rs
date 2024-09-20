use aws_sdk_eventbridge::types::PutEventsRequestEntry; 
use lambda_runtime::{tracing, Error};


// Funcion que emite un evento a un bus de EventBridge
pub async fn emit_event(event_str: &str, event_name: &str, source_origin: &str, eventbus_name: &str) -> Result<(), Error> {
    // Aqui va la logica para emitir el evento al bus por defecto de 
    //todo!("Implementar la lógica para emitir el evento al bus por defecto de EventBridge.");
    
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_eventbridge::Client::new(&config);

    let event_entry = PutEventsRequestEntry::builder()
        .detail_type(event_name) 
        .source(source_origin) 
        .detail(event_str)
        .event_bus_name(eventbus_name)
        .build();
    tracing::info!("Se emitirá un evento en el bus por defecto de EventBridge: {:?}", event_entry);

    let response = client.put_events().entries(event_entry).send().await?;

    tracing::info!("Evento emitido correctamente {:?}", response);

    Ok(())

}
