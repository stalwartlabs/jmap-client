pub mod get;
pub mod helpers;
pub mod set;

use std::fmt::Display;

use crate::core::set::date_not_set;
use crate::core::set::string_not_set;
use crate::core::Type;
use crate::Get;
use crate::Set;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VacationResponse<State = Get> {
    #[serde(skip)]
    _create_id: Option<usize>,

    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(rename = "isEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    is_enabled: Option<bool>,

    #[serde(rename = "fromDate")]
    #[serde(skip_serializing_if = "date_not_set")]
    from_date: Option<DateTime<Utc>>,

    #[serde(rename = "toDate")]
    #[serde(skip_serializing_if = "date_not_set")]
    to_date: Option<DateTime<Utc>>,

    #[serde(rename = "subject")]
    #[serde(skip_serializing_if = "string_not_set")]
    subject: Option<String>,

    #[serde(rename = "textBody")]
    #[serde(skip_serializing_if = "string_not_set")]
    text_body: Option<String>,

    #[serde(rename = "htmlBody")]
    #[serde(skip_serializing_if = "string_not_set")]
    html_body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum Property {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "isEnabled")]
    IsEnabled,
    #[serde(rename = "fromDate")]
    FromDate,
    #[serde(rename = "toDate")]
    ToDate,
    #[serde(rename = "subject")]
    Subject,
    #[serde(rename = "textBody")]
    TextBody,
    #[serde(rename = "htmlBody")]
    HtmlBody,
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Id => write!(f, "id"),
            Property::IsEnabled => write!(f, "isEnabled"),
            Property::FromDate => write!(f, "fromDate"),
            Property::ToDate => write!(f, "toDate"),
            Property::Subject => write!(f, "subject"),
            Property::TextBody => write!(f, "textBody"),
            Property::HtmlBody => write!(f, "htmlBody"),
        }
    }
}

impl Type for VacationResponse<Set> {
    fn requires_account_id() -> bool {
        true
    }
}

impl Type for Property {
    fn requires_account_id() -> bool {
        true
    }
}
