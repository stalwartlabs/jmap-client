use std::collections::HashMap;

use serde::Serialize;

use crate::{Method, URI};

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    using: Vec<URI>,
    #[serde(rename = "methodCalls")]
    method_calls: Vec<(Method, Arguments, String)>,
    #[serde(rename = "createdIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    created_ids: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResultReference {
    #[serde(rename = "resultOf")]
    result_of: String,
    name: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Arguments {}
