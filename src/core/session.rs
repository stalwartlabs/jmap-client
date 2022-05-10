use std::collections::HashMap;

use serde::Deserialize;

use crate::email::{MailCapabilities, SubmissionCapabilities};

#[derive(Debug, Clone, Deserialize)]
pub struct Session {
    #[serde(rename = "capabilities")]
    capabilities: HashMap<String, Capabilities>,

    #[serde(rename = "accounts")]
    accounts: HashMap<String, Account>,

    #[serde(rename = "primaryAccounts")]
    primary_accounts: HashMap<String, String>,

    #[serde(rename = "username")]
    username: String,

    #[serde(rename = "apiUrl")]
    api_url: String,

    #[serde(rename = "downloadUrl")]
    download_url: String,

    #[serde(rename = "uploadUrl")]
    upload_url: String,

    #[serde(rename = "eventSourceUrl")]
    event_source_url: String,

    #[serde(rename = "state")]
    state: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Account {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "isPersonal")]
    is_personal: bool,

    #[serde(rename = "isReadOnly")]
    is_read_only: bool,

    #[serde(rename = "accountCapabilities")]
    account_capabilities: HashMap<String, Capabilities>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Capabilities {
    Core(CoreCapabilities),
    Mail(MailCapabilities),
    Submission(SubmissionCapabilities),
    Empty(EmptyCapabilities),
    Other(serde_json::Value),
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoreCapabilities {
    #[serde(rename = "maxSizeUpload")]
    max_size_upload: usize,

    #[serde(rename = "maxConcurrentUpload")]
    max_concurrent_upload: usize,

    #[serde(rename = "maxSizeRequest")]
    max_size_request: usize,

    #[serde(rename = "maxConcurrentRequests")]
    max_concurrent_requests: usize,

    #[serde(rename = "maxCallsInRequest")]
    max_calls_in_request: usize,

    #[serde(rename = "maxObjectsInGet")]
    max_objects_in_get: usize,

    #[serde(rename = "maxObjectsInSet")]
    max_objects_in_set: usize,

    #[serde(rename = "collationAlgorithms")]
    collation_algorithms: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmptyCapabilities {}

impl Session {
    pub fn capabilities(&self) -> impl Iterator<Item = &String> {
        self.capabilities.keys()
    }

    pub fn capability(&self, capability: &str) -> Option<&Capabilities> {
        self.capabilities.get(capability)
    }

    pub fn accounts(&self) -> impl Iterator<Item = &String> {
        self.accounts.keys()
    }

    pub fn account(&self, account: &str) -> Option<&Account> {
        self.accounts.get(account)
    }

    pub fn primary_accounts(&self) -> impl Iterator<Item = &String> {
        self.primary_accounts.keys()
    }

    pub fn primary_account(&self, account: &str) -> Option<&String> {
        self.primary_accounts.get(account)
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn api_url(&self) -> &str {
        &self.api_url
    }

    pub fn download_url(&self) -> &str {
        &self.download_url
    }

    pub fn upload_url(&self) -> &str {
        &self.upload_url
    }

    pub fn event_source_url(&self) -> &str {
        &self.event_source_url
    }

    pub fn state(&self) -> &str {
        &self.state
    }
}

impl Account {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_personal(&self) -> bool {
        self.is_personal
    }

    pub fn is_read_only(&self) -> bool {
        self.is_read_only
    }

    pub fn capabilities(&self) -> impl Iterator<Item = &String> {
        self.account_capabilities.keys()
    }

    pub fn capability(&self, capability: &str) -> Option<&Capabilities> {
        self.account_capabilities.get(capability)
    }
}

impl CoreCapabilities {
    pub fn max_size_upload(&self) -> usize {
        self.max_size_upload
    }

    pub fn max_concurrent_upload(&self) -> usize {
        self.max_concurrent_upload
    }

    pub fn max_size_request(&self) -> usize {
        self.max_size_request
    }

    pub fn max_concurrent_requests(&self) -> usize {
        self.max_concurrent_requests
    }

    pub fn max_calls_in_request(&self) -> usize {
        self.max_calls_in_request
    }

    pub fn max_objects_in_get(&self) -> usize {
        self.max_objects_in_get
    }

    pub fn max_objects_in_set(&self) -> usize {
        self.max_objects_in_set
    }

    pub fn collation_algorithms(&self) -> &[String] {
        &self.collation_algorithms
    }
}
