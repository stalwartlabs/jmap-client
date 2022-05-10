use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::core::set::SetError;

#[derive(Debug, Clone, Serialize)]
pub struct CopyBlobRequest {
    #[serde(rename = "fromAccountId")]
    from_account_id: String,
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "blobIds")]
    blob_ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CopyBlobResponse {
    #[serde(rename = "fromAccountId")]
    from_account_id: String,
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "copied")]
    copied: Option<HashMap<String, String>>,
    #[serde(rename = "notCopied")]
    not_copied: Option<HashMap<String, SetError<String>>>,
}

impl CopyBlobRequest {
    pub fn new(from_account_id: String, account_id: String) -> Self {
        CopyBlobRequest {
            from_account_id,
            account_id,
            blob_ids: vec![],
        }
    }

    pub fn blob_id(&mut self, blob_id: String) -> &mut Self {
        self.blob_ids.push(blob_id);
        self
    }
}

impl CopyBlobResponse {
    pub fn from_account_id(&self) -> &str {
        &self.from_account_id
    }

    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    pub fn copied(&self) -> Option<impl Iterator<Item = &String>> {
        self.copied.as_ref().map(|map| map.keys())
    }

    pub fn copied_details(&self, id: &str) -> Option<&str> {
        self.copied
            .as_ref()
            .and_then(|map| map.get(id).map(|s| s.as_str()))
    }

    pub fn not_copied(&self) -> Option<impl Iterator<Item = &String>> {
        self.not_copied.as_ref().map(|map| map.keys())
    }

    pub fn not_copied_reason(&self, id: &str) -> Option<&SetError<String>> {
        self.not_copied.as_ref().and_then(|map| map.get(id))
    }
}
