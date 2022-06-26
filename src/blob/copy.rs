use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    core::{set::SetError, RequestParams},
    Error,
};

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
    pub fn new(params: RequestParams, from_account_id: impl Into<String>) -> Self {
        CopyBlobRequest {
            from_account_id: from_account_id.into(),
            account_id: params.account_id,
            blob_ids: vec![],
        }
    }

    pub fn blob_id(&mut self, blob_id: impl Into<String>) -> &mut Self {
        self.blob_ids.push(blob_id.into());
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

    pub fn copied(&mut self, id: &str) -> crate::Result<String> {
        if let Some(result) = self.copied.as_mut().and_then(|r| r.remove(id)) {
            Ok(result)
        } else if let Some(error) = self.not_copied.as_mut().and_then(|r| r.remove(id)) {
            Err(error.to_string_error().into())
        } else {
            Err(Error::Internal(format!("Id {} not found.", id)))
        }
    }

    pub fn copied_ids(&self) -> Option<impl Iterator<Item = &String>> {
        self.copied.as_ref().map(|map| map.keys())
    }

    pub fn not_copied_ids(&self) -> Option<impl Iterator<Item = &String>> {
        self.not_copied.as_ref().map(|map| map.keys())
    }

    pub fn not_copied_reason(&self, id: &str) -> Option<&SetError<String>> {
        self.not_copied.as_ref().and_then(|map| map.get(id))
    }
}
