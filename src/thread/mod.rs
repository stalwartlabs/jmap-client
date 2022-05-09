pub mod get;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thread {
    id: String,
    #[serde(rename = "emailIds")]
    email_ids: Vec<String>,
}
