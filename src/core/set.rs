use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct SetRequest<T, A: Default> {
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "ifInState")]
    if_in_state: Option<String>,
    create: Option<HashMap<String, T>>,
    update: Option<HashMap<String, T>>,
    destroy: Option<Vec<String>>,

    #[serde(flatten)]
    arguments: A,
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

impl<T, A: Default> SetRequest<T, A> {
    pub fn new(account_id: String) -> Self {
        Self {
            account_id,
            if_in_state: None,
            create: None,
            update: None,
            destroy: None,
            arguments: Default::default(),
        }
    }

    pub fn account_id(&mut self, account_id: impl Into<String>) -> &mut Self {
        self.account_id = account_id.into();
        self
    }

    pub fn if_in_state(&mut self, if_in_state: impl Into<String>) -> &mut Self {
        self.if_in_state = Some(if_in_state.into());
        self
    }

    pub fn create(&mut self, id: impl Into<String>, value: T) -> &mut Self {
        self.create
            .get_or_insert_with(HashMap::new)
            .insert(id.into(), value);
        self
    }

    pub fn update(&mut self, id: impl Into<String>, value: T) -> &mut Self {
        self.update
            .get_or_insert_with(HashMap::new)
            .insert(id.into(), value);
        self
    }

    pub fn destroy(&mut self, id: impl Into<String>) -> &mut Self {
        self.destroy.get_or_insert_with(Vec::new).push(id.into());
        self
    }

    pub fn arguments(&mut self) -> &mut A {
        &mut self.arguments
    }
}

impl<T, U> SetResponse<T, U> {
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

    pub fn updated(&self) -> Option<impl Iterator<Item = &String>> {
        self.updated.as_ref().map(|map| map.keys())
    }

    pub fn destroyed(&self) -> Option<&[String]> {
        self.destroyed.as_deref()
    }

    pub fn not_created(&self) -> Option<impl Iterator<Item = &String>> {
        self.not_created.as_ref().map(|map| map.keys())
    }

    pub fn not_updated(&self) -> Option<impl Iterator<Item = &String>> {
        self.not_updated.as_ref().map(|map| map.keys())
    }

    pub fn not_destroyed(&self) -> Option<impl Iterator<Item = &String>> {
        self.not_destroyed.as_ref().map(|map| map.keys())
    }

    pub fn not_created_reason(&self, id: &str) -> Option<&SetError<U>> {
        self.not_created.as_ref().and_then(|map| map.get(id))
    }

    pub fn not_updated_reason(&self, id: &str) -> Option<&SetError<U>> {
        self.not_updated.as_ref().and_then(|map| map.get(id))
    }

    pub fn not_destroyed_reason(&self, id: &str) -> Option<&SetError<U>> {
        self.not_destroyed.as_ref().and_then(|map| map.get(id))
    }

    pub fn created_details(&self, id: &str) -> Option<&T> {
        self.created.as_ref().and_then(|map| map.get(id))
    }

    pub fn updated_details(&self, id: &str) -> Option<&T> {
        self.updated.as_ref().and_then(|map| map.get(id))?.as_ref()
    }
}

impl<U> SetError<U> {
    pub fn error(&self) -> &SetErrorType {
        &self.type_
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn properties(&self) -> Option<&[U]> {
        self.properties.as_deref()
    }
}
