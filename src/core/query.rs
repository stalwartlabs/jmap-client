use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct QueryRequest<T, U> {
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "filter")]
    filter: Option<Filter<T>>,
    #[serde(rename = "sort")]
    sort: Option<Vec<U>>,
    #[serde(rename = "position")]
    position: i32,
    #[serde(rename = "anchor")]
    anchor: Option<String>,
    #[serde(rename = "anchorOffset")]
    anchor_offset: i32,
    #[serde(rename = "limit")]
    limit: Option<usize>,
    #[serde(rename = "calculateTotal")]
    calculate_total: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Filter<T> {
    FilterOperator(FilterOperator<T>),
    FilterCondition(T),
}

#[derive(Debug, Clone, Serialize)]
pub struct FilterOperator<T> {
    operator: Operator,
    conditions: Vec<Filter<T>>,
}

#[derive(Debug, Clone, Serialize)]
pub enum Operator {
    #[serde(rename = "AND")]
    And,
    #[serde(rename = "OR")]
    Or,
    #[serde(rename = "NOT")]
    Not,
}

#[derive(Debug, Clone, Serialize)]
pub struct Comparator<T> {
    property: T,
    #[serde(rename = "isAscending")]
    is_ascending: bool,
    collation: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct QueryResponse {
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "queryState")]
    query_state: String,
    #[serde(rename = "canCalculateChanges")]
    can_calculate_changes: bool,
    #[serde(rename = "position")]
    position: i32,
    #[serde(rename = "ids")]
    ids: Vec<String>,
    #[serde(rename = "total")]
    total: Option<usize>,
    #[serde(rename = "limit")]
    limit: Option<usize>,
}
