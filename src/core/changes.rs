use serde::{Deserialize, Serialize};

use super::{Object, RequestParams};

pub trait ChangesObject: Object {
    type ChangesResponse;
}

#[derive(Debug, Clone, Serialize)]
pub struct ChangesRequest {
    #[serde(rename = "accountId")]
    account_id: String,

    #[serde(rename = "sinceState")]
    since_state: String,

    #[serde(rename = "maxChanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    max_changes: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangesResponse<O: ChangesObject> {
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

    #[serde(flatten)]
    arguments: O::ChangesResponse,
}

impl ChangesRequest {
    pub fn new(params: RequestParams, since_state: String) -> Self {
        ChangesRequest {
            account_id: params.account_id,
            since_state,
            max_changes: None,
        }
    }

    pub fn account_id(&mut self, account_id: impl Into<String>) -> &mut Self {
        self.account_id = account_id.into();
        self
    }

    pub fn max_changes(&mut self, max_changes: usize) -> &mut Self {
        self.max_changes = Some(max_changes);
        self
    }
}

impl<O: ChangesObject> ChangesResponse<O> {
    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    pub fn unwrap_account_id(self) -> String {
        self.account_id
    }

    pub fn old_state(&self) -> &str {
        &self.old_state
    }

    pub fn new_state(&self) -> &str {
        &self.new_state
    }

    pub fn unwrap_new_state(self) -> String {
        self.new_state
    }

    pub fn has_more_changes(&self) -> bool {
        self.has_more_changes
    }

    pub fn created(&self) -> &[String] {
        &self.created
    }

    pub fn updated(&self) -> &[String] {
        &self.updated
    }

    pub fn destroyed(&self) -> &[String] {
        &self.destroyed
    }

    pub fn arguments(&self) -> &O::ChangesResponse {
        &self.arguments
    }

    pub fn total_changes(&self) -> usize {
        self.created.len() + self.updated.len() + self.destroyed.len()
    }
}
