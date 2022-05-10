use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::core::set::{from_timestamp, SetError};

use super::{Email, Property};

#[derive(Debug, Clone, Serialize)]
pub struct EmailImportRequest {
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "ifInState")]
    if_in_state: Option<String>,

    emails: HashMap<String, EmailImport>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EmailImport {
    #[serde(rename = "blobId")]
    blob_id: String,

    #[serde(rename = "mailboxIds")]
    mailbox_ids: HashMap<String, bool>,

    #[serde(rename = "keywords")]
    keywords: HashMap<String, bool>,

    #[serde(rename = "receivedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    received_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailImportResponse {
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "oldState")]
    old_state: Option<String>,
    #[serde(rename = "newState")]
    new_state: String,
    #[serde(rename = "created")]
    created: Option<HashMap<String, Email>>,
    #[serde(rename = "notCreated")]
    not_created: Option<HashMap<String, SetError<Property>>>,
}

impl EmailImportRequest {
    pub fn new(account_id: String) -> Self {
        EmailImportRequest {
            account_id,
            if_in_state: None,
            emails: HashMap::new(),
        }
    }

    pub fn if_in_state(&mut self, if_in_state: String) -> &mut Self {
        self.if_in_state = Some(if_in_state);
        self
    }

    pub fn add_email(&mut self, id: String, email_import: EmailImport) -> &mut Self {
        self.emails.insert(id, email_import);
        self
    }
}

impl EmailImport {
    pub fn new(blob_id: String) -> Self {
        EmailImport {
            blob_id,
            mailbox_ids: HashMap::new(),
            keywords: HashMap::new(),
            received_at: None,
        }
    }

    pub fn mailbox_id(mut self, mailbox_id: String) -> Self {
        self.mailbox_ids.insert(mailbox_id, true);
        self
    }

    pub fn keyword(mut self, keyword: String) -> Self {
        self.keywords.insert(keyword, true);
        self
    }

    pub fn received_at(mut self, received_at: i64) -> Self {
        self.received_at = Some(from_timestamp(received_at));
        self
    }
}

impl EmailImportResponse {
    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    pub fn old_state(&self) -> Option<&str> {
        self.old_state.as_deref()
    }

    pub fn new_state(&self) -> &str {
        &self.new_state
    }

    pub fn created(&self) -> Option<impl Iterator<Item = &String>> {
        self.created.as_ref().map(|map| map.keys())
    }

    pub fn not_created(&self) -> Option<impl Iterator<Item = &String>> {
        self.not_created.as_ref().map(|map| map.keys())
    }

    pub fn created_details(&self, id: &str) -> Option<&Email> {
        self.created.as_ref().and_then(|map| map.get(id))
    }

    pub fn not_created_reason(&self, id: &str) -> Option<&SetError<Property>> {
        self.not_created.as_ref().and_then(|map| map.get(id))
    }
}
