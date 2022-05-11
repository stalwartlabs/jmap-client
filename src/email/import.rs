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
    #[serde(skip_serializing_if = "Option::is_none")]
    if_in_state: Option<String>,

    emails: HashMap<String, EmailImport>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EmailImport {
    #[serde(skip)]
    create_id: usize,

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

    pub fn if_in_state(&mut self, if_in_state: impl Into<String>) -> &mut Self {
        self.if_in_state = Some(if_in_state.into());
        self
    }

    pub fn email(&mut self, blob_id: impl Into<String>) -> &mut EmailImport {
        let create_id = self.emails.len();
        let create_id_str = format!("i{}", create_id);
        self.emails.insert(
            create_id_str.clone(),
            EmailImport::new(blob_id.into(), create_id),
        );
        self.emails.get_mut(&create_id_str).unwrap()
    }
}

impl EmailImport {
    fn new(blob_id: String, create_id: usize) -> Self {
        EmailImport {
            create_id,
            blob_id,
            mailbox_ids: HashMap::new(),
            keywords: HashMap::new(),
            received_at: None,
        }
    }

    pub fn mailbox_id(&mut self, mailbox_id: impl Into<String>) -> &mut Self {
        self.mailbox_ids.insert(mailbox_id.into(), true);
        self
    }

    pub fn keyword(&mut self, keyword: impl Into<String>) -> &mut Self {
        self.keywords.insert(keyword.into(), true);
        self
    }

    pub fn received_at(&mut self, received_at: i64) -> &mut Self {
        self.received_at = Some(from_timestamp(received_at));
        self
    }

    pub fn create_id(&self) -> String {
        format!("i{}", self.create_id)
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
