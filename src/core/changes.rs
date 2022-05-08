use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct ChangesRequest {
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "sinceState")]
    since_state: String,
    #[serde(rename = "maxChanges")]
    max_changes: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangesResponse {
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "oldState")]
    old_state: String,
    #[serde(rename = "newState")]
    new_state: String,
    #[serde(rename = "hasMoreChanges")]
    has_more_changes: bool,
    created: Vec<String>,
    updated: Vec<String>,
    destroyed: Vec<String>,
}
