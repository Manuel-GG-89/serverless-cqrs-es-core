use std::collections::HashMap;

use aws_sdk_dynamodb::{types::AttributeValue, Client};
use aws_config::meta::region::RegionProviderChain;
use lambda_runtime::tracing;
use serde_json::{Error, Value};

pub mod appsync_structs;
pub mod event_emitter;
pub mod current_state;


pub async fn test() {
    tracing::info!("Hello, world!");
}

pub async fn get_dynamodb_client() -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else("sa-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    Client::new(&config)
}

pub fn convert_to_attribute_value(value: Value) -> AttributeValue {
    match value {
        Value::String(s) => AttributeValue::S(s),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                AttributeValue::N(i.to_string())
            } else if let Some(f) = n.as_f64() {
                AttributeValue::N(f.to_string())
            } else {
                AttributeValue::N(n.to_string())
            }
        },
        Value::Bool(b) => AttributeValue::Bool(b),
        Value::Array(arr) => {
            let vec = arr.into_iter().map(convert_to_attribute_value).collect();
            AttributeValue::L(vec)
        },
        Value::Object(map) => {
            let mut attr_map = HashMap::new();
            for (k, v) in map {
                attr_map.insert(k, convert_to_attribute_value(v));
            }
            AttributeValue::M(attr_map)
        },
        _ => AttributeValue::Null(true),
    }
}

pub fn convert_json_value_to_hashmap(event_detail: Value) -> Result<HashMap<String, AttributeValue>, Error>{
    // Convert JSON value to a HashMap of AttributeValues
    let mut item = HashMap::new();
    if let Value::Object(map) = event_detail {
        for (k, v) in map {
            item.insert(k, convert_to_attribute_value(v));
        }
    }
    Ok(item)
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        todo!("Implement tests");
    }
}
