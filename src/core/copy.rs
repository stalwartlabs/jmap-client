use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::set::SetError;

#[derive(Debug, Clone, Serialize)]
pub struct CopyRequest<T> {
    #[serde(rename = "fromAccountId")]
    from_account_id: String,
    #[serde(rename = "ifFromInState")]
    if_from_in_state: Option<String>,
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "ifInState")]
    if_in_state: Option<String>,
    #[serde(rename = "create")]
    create: HashMap<String, T>,
    #[serde(rename = "onSuccessDestroyOriginal")]
    on_success_destroy_original: bool,
    #[serde(rename = "destroyFromIfInState")]
    destroy_from_if_in_state: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CopyResponse<T, U> {
    #[serde(rename = "fromAccountId")]
    from_account_id: String,
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "oldState")]
    old_state: Option<String>,
    #[serde(rename = "newState")]
    new_state: String,
    #[serde(rename = "created")]
    created: Option<HashMap<String, T>>,
    #[serde(rename = "notCreated")]
    not_created: Option<HashMap<String, SetError<U>>>,
}
