pub mod get;
#[cfg(feature = "async")]
pub mod helpers;
#[cfg(feature = "blocking")]
pub mod helpers_blocking;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::core::{changes::ChangesObject, Object};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thread {
    id: String,
    #[serde(rename = "emailIds")]
    email_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum Property {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "emailIds")]
    EmailIds,
}

impl Object for Thread {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Id => write!(f, "id"),
            Property::EmailIds => write!(f, "emailIds"),
        }
    }
}

impl ChangesObject for Thread {
    type ChangesResponse = ();
}
