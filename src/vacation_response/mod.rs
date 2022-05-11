pub mod get;
pub mod set;

use crate::core::set::date_not_set;
use crate::core::set::string_not_set;
use crate::Get;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
