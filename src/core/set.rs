use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, Serialize)]
pub struct SetRequest<T, U>
where
    U: Eq + Hash,
{
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "ifInState")]
    if_in_state: Option<String>,
    create: Option<HashMap<String, T>>,
    update: Option<HashMap<String, HashMap<U, serde_json::Value>>>,
    destroy: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetResponse<T, U> {
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "oldState")]
    old_state: Option<String>,
    #[serde(rename = "newState")]
    new_state: String,
    #[serde(rename = "created")]
    created: Option<HashMap<String, T>>,
    #[serde(rename = "updated")]
    updated: Option<HashMap<String, Option<T>>>,
    #[serde(rename = "destroyed")]
    destroyed: Option<Vec<String>>,
    #[serde(rename = "notCreated")]
    not_created: Option<HashMap<String, SetError<U>>>,
    #[serde(rename = "notUpdated")]
    not_updated: Option<HashMap<String, SetError<U>>>,
    #[serde(rename = "notDestroyed")]
    not_destroyed: Option<HashMap<String, SetError<U>>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetError<U> {
    #[serde(rename = "type")]
    type_: SetErrorType,
    description: Option<String>,
    properties: Option<Vec<U>>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum SetErrorType {
    #[serde(rename = "forbidden")]
    Forbidden,
    #[serde(rename = "overQuota")]
    OverQuota,
    #[serde(rename = "tooLarge")]
    TooLarge,
    #[serde(rename = "rateLimit")]
    RateLimit,
    #[serde(rename = "notFound")]
    NotFound,
    #[serde(rename = "invalidPatch")]
    InvalidPatch,
    #[serde(rename = "willDestroy")]
    WillDestroy,
    #[serde(rename = "invalidProperties")]
    InvalidProperties,
    #[serde(rename = "singleton")]
    Singleton,
    #[serde(rename = "mailboxHasChild")]
    MailboxHasChild,
    #[serde(rename = "mailboxHasEmail")]
    MailboxHasEmail,
    #[serde(rename = "blobNotFound")]
    BlobNotFound,
    #[serde(rename = "tooManyKeywords")]
    TooManyKeywords,
    #[serde(rename = "tooManyMailboxes")]
    TooManyMailboxes,
    #[serde(rename = "forbiddenFrom")]
    ForbiddenFrom,
    #[serde(rename = "invalidEmail")]
    InvalidEmail,
    #[serde(rename = "tooManyRecipients")]
    TooManyRecipients,
    #[serde(rename = "noRecipients")]
    NoRecipients,
    #[serde(rename = "invalidRecipients")]
    InvalidRecipients,
    #[serde(rename = "forbiddenMailFrom")]
    ForbiddenMailFrom,
    #[serde(rename = "forbiddenToSend")]
    ForbiddenToSend,
    #[serde(rename = "cannotUnsend")]
    CannotUnsend,
}

pub fn from_timestamp(timestamp: i64) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc)
}

pub fn string_not_set(string: &Option<String>) -> bool {
    matches!(string, Some(string) if string.is_empty())
}

pub fn date_not_set(date: &Option<DateTime<Utc>>) -> bool {
    matches!(date, Some(date) if date.timestamp() == 0)
}

pub fn list_not_set<T>(list: &Option<Vec<T>>) -> bool {
    matches!(list, Some(list) if list.is_empty() )
}
