use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, Serialize)]
pub struct SetRequest<T, U>
where
    U: Eq + Hash,
{
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "ifInState")]
    if_in_state: Option<String>,
    create: Option<HashMap<String, T>>,
    update: Option<HashMap<String, HashMap<U, serde_json::Value>>>,
    destroy: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetResponse<T, U> {
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "oldState")]
    old_state: Option<String>,
    #[serde(rename = "newState")]
    new_state: String,
    #[serde(rename = "created")]
    created: Option<HashMap<String, T>>,
    #[serde(rename = "updated")]
    updated: Option<HashMap<String, Option<T>>>,
    #[serde(rename = "destroyed")]
    destroyed: Option<Vec<String>>,
    #[serde(rename = "notCreated")]
    not_created: Option<HashMap<String, SetError<U>>>,
    #[serde(rename = "notUpdated")]
    not_updated: Option<HashMap<String, SetError<U>>>,
    #[serde(rename = "notDestroyed")]
    not_destroyed: Option<HashMap<String, SetError<U>>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetError<U> {
    #[serde(rename = "type")]
    type_: SetErrorType,
    description: Option<String>,
    properties: Option<Vec<U>>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum SetErrorType {
    #[serde(rename = "forbidden")]
    Forbidden,
    #[serde(rename = "overQuota")]
    OverQuota,
    #[serde(rename = "tooLarge")]
    TooLarge,
    #[serde(rename = "rateLimit")]
    RateLimit,
    #[serde(rename = "notFound")]
    NotFound,
    #[serde(rename = "invalidPatch")]
    InvalidPatch,
    #[serde(rename = "willDestroy")]
    WillDestroy,
    #[serde(rename = "invalidProperties")]
    InvalidProperties,
    #[serde(rename = "singleton")]
    Singleton,
}
