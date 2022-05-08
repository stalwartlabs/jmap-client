use std::collections::HashMap;

use serde::Deserialize;

use crate::Method;

#[derive(Debug, Clone, Deserialize)]
pub struct Request {
    #[serde(rename = "methodResponses")]
    method_calls: Vec<(Method, Result, String)>,
    #[serde(rename = "createdIds")]
    created_ids: Option<HashMap<String, String>>,
    #[serde(rename = "sessionState")]
    session_state: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Result {}
