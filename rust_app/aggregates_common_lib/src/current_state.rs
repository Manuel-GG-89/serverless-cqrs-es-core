use std::collections::HashMap;
use lambda_runtime::{tracing, Error};
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::types::AttributeValue;
use serde_json::{json, Value};

pub async fn get_aggregate_state(dynamodb_client: &Client, table_name: &str ,id: &str) -> Result<Value, Error> {

    // Imprimir cada parametro recibido
    tracing::info!("table_name: {}", table_name);
    tracing::info!("id: {}", id);
    
    // Consulta a la tabla de DynamoDB
    let result = dynamodb_client.query()
        .table_name(table_name)
        .key_condition_expression("#id = :id_val")
        .expression_attribute_names("#id", "id")
        .expression_attribute_values(":id_val", AttributeValue::S(id.to_string()))
        .send().await?;

    if result.scanned_count() == 0 {
        return Err(
            Error::from(format!("No se encuentra el agregado, no hay eventos registrados para el agregado con order_id: {}", id))
        );
    } 
  
    let items = result.items.unwrap_or_default();

    let mut aggregate_state = json!({});

    let default_str_value = "".to_string();

    items.iter().for_each(|item| {
        let event: HashMap<String, AttributeValue> = item.clone();
        for (key, value) in event {
            match value {
                AttributeValue::S(s) => {
                    aggregate_state[key] = json!(Value::String(s));
                }
                AttributeValue::N(n) => {
                    aggregate_state[key] = json!(Value::Number(n.parse::<i32>().unwrap().into()));
                }
                _ => {
                    aggregate_state[key] = json!(Value::String(default_str_value.clone()));
                }
            }
        }
    });

    Ok(aggregate_state)
}