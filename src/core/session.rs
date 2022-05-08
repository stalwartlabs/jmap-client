use std::collections::HashMap;

use serde::Deserialize;

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
enum Capabilities {
    Core(CoreCapabilities),
    Mail(MailCapabilities),
    Submission(SubmissionCapabilities),
    EmptyCapabilities(EmptyCapabilities),
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
pub struct MailCapabilities {
    #[serde(rename = "maxMailboxesPerEmail")]
    max_mailboxes_per_email: Option<usize>,
    #[serde(rename = "maxMailboxDepth")]
    max_mailbox_depth: usize,
    #[serde(rename = "maxSizeMailboxName")]
    max_size_mailbox_name: usize,
    #[serde(rename = "maxSizeAttachmentsPerEmail")]
    max_size_attachments_per_email: usize,
    #[serde(rename = "emailQuerySortOptions")]
    email_query_sort_options: Vec<String>,
    #[serde(rename = "mayCreateTopLevelMailbox")]
    may_create_top_level_mailbox: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmissionCapabilities {
    #[serde(rename = "maxDelayedSend")]
    max_delayed_send: usize,
    #[serde(rename = "submissionExtensions")]
    submission_extensions: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmptyCapabilities {}
