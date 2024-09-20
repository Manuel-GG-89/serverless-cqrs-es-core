use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppsyncEvent {
    pub arguments: Arguments,
    pub identity: Identity,
    pub source: serde_json::Value, 
    pub info: Info,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Arguments {
    pub operation: serde_json::Value, 
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub username: String,
    pub source_ip: Vec<String>,
    pub user_agent: String,
    pub groups: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Headers {
    #[serde(rename = "x-api-key")]
    pub x_api_key: String,
    #[serde(rename = "x-amz-date")]
    pub x_amz_date: String,
    pub authorization: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub field_name: String,
    pub parent_type_name: String,
    pub variables: serde_json::Value, 
}
