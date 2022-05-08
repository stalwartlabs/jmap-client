use serde::{Deserialize, Serialize};

use super::query::Filter;

#[derive(Debug, Clone, Serialize)]
pub struct QueryChangesRequest<T, U> {
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "filter")]
    filter: Option<Filter<T>>,
    #[serde(rename = "sort")]
    sort: Option<Vec<U>>,
    #[serde(rename = "sinceQueryState")]
    since_query_state: String,
    #[serde(rename = "maxChanges")]
    max_changes: Option<usize>,
    #[serde(rename = "upToId")]
    up_to_id: Option<String>,
    #[serde(rename = "calculateTotal")]
    calculate_total: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct QueryChangesResponse {
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "oldQueryState")]
    old_query_state: String,
    #[serde(rename = "newQueryState")]
    new_query_state: String,
    #[serde(rename = "total")]
    total: Option<usize>,
    #[serde(rename = "removed")]
    removed: Vec<String>,
    #[serde(rename = "added")]
    added: Vec<AddedItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddedItem {
    id: String,
    index: usize,
}
