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
pub struct CopyBlobResponse<U> {
    #[serde(rename = "fromAccountId")]
    from_account_id: String,
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "copied")]
    copied: Option<HashMap<String, String>>,
    #[serde(rename = "notCopied")]
    not_copied: Option<HashMap<String, SetError<U>>>,
}
