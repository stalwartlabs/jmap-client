use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct GetRequest<T> {
    #[serde(rename = "accountId")]
    account_id: String,
    ids: Option<Vec<String>>,
    properties: Option<Vec<T>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetResponse<T> {
    #[serde(rename = "accountId")]
    account_id: String,
    state: String,
    list: Vec<T>,
    #[serde(rename = "notFound")]
    not_found: Vec<String>,
}
