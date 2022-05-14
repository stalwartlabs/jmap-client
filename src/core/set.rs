use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

use crate::Error;

use super::{request::ResultReference, RequestParams};

#[derive(Debug, Clone, Serialize)]
pub struct SetRequest<T: Create, A: Default> {
    #[serde(rename = "accountId")]
    account_id: String,

    #[serde(rename = "ifInState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    if_in_state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<HashMap<String, T>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<HashMap<String, T>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    destroy: Option<Vec<String>>,

    #[serde(rename = "#destroy")]
    #[serde(skip_deserializing)]
    #[serde(skip_serializing_if = "Option::is_none")]
    destroy_ref: Option<ResultReference>,

    #[serde(flatten)]
    arguments: A,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetResponse<T, U: Display> {
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
pub struct SetError<U>
where
    U: Display,
{
    #[serde(rename = "type")]
    pub type_: SetErrorType,
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

pub trait Create: Sized {
    fn new(create_id: Option<usize>) -> Self;
    fn create_id(&self) -> Option<String>;
}

impl<T: Create, A: Default> SetRequest<T, A> {
    pub fn new(params: RequestParams) -> Self {
        Self {
            account_id: params.account_id,
            if_in_state: None,
            create: None,
            update: None,
            destroy: None,
            destroy_ref: None,
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

    pub fn create(&mut self) -> &mut T {
        let create_id = self.create.as_ref().map_or(0, |c| c.len());
        let create_id_str = format!("c{}", create_id);
        self.create
            .get_or_insert_with(HashMap::new)
            .insert(create_id_str.clone(), T::new(create_id.into()));
        self.create
            .as_mut()
            .unwrap()
            .get_mut(&create_id_str)
            .unwrap()
    }

    pub fn create_item(&mut self, item: T) -> String {
        let create_id = self.create.as_ref().map_or(0, |c| c.len());
        let create_id_str = format!("c{}", create_id);
        self.create
            .get_or_insert_with(HashMap::new)
            .insert(create_id_str.clone(), item);
        create_id_str
    }

    pub fn update(&mut self, id: impl Into<String>) -> &mut T {
        let id: String = id.into();
        self.update
            .get_or_insert_with(HashMap::new)
            .insert(id.clone(), T::new(None));
        self.update.as_mut().unwrap().get_mut(&id).unwrap()
    }

    pub fn update_item(&mut self, id: impl Into<String>, item: T) {
        self.update
            .get_or_insert_with(HashMap::new)
            .insert(id.into(), item);
    }

    pub fn destroy<U, V>(&mut self, ids: U) -> &mut Self
    where
        U: IntoIterator<Item = V>,
        V: Into<String>,
    {
        self.destroy
            .get_or_insert_with(Vec::new)
            .extend(ids.into_iter().map(|id| id.into()));
        self.destroy_ref = None;
        self
    }

    pub fn destroy_ref(&mut self, reference: ResultReference) -> &mut Self {
        self.destroy_ref = reference.into();
        self.destroy = None;
        self
    }

    pub fn arguments(&mut self) -> &mut A {
        &mut self.arguments
    }
}

impl<T, U: Display> SetResponse<T, U> {
    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    pub fn old_state(&self) -> Option<&str> {
        self.old_state.as_deref()
    }

    pub fn new_state(&self) -> &str {
        &self.new_state
    }

    pub fn created(&mut self, id: &str) -> crate::Result<T> {
        if let Some(result) = self.created.as_mut().and_then(|r| r.remove(id)) {
            Ok(result)
        } else if let Some(error) = self.not_created.as_mut().and_then(|r| r.remove(id)) {
            Err(error.to_string_error().into())
        } else {
            Err(Error::Internal(format!("Id {} not found.", id)))
        }
    }

    pub fn updated(&mut self, id: &str) -> crate::Result<Option<T>> {
        if let Some(result) = self.updated.as_mut().and_then(|r| r.remove(id)) {
            Ok(result)
        } else if let Some(error) = self.not_updated.as_mut().and_then(|r| r.remove(id)) {
            Err(error.to_string_error().into())
        } else {
            Err(Error::Internal(format!("Id {} not found.", id)))
        }
    }

    pub fn destroyed(&mut self, id: &str) -> crate::Result<()> {
        if self
            .destroyed
            .as_ref()
            .map_or(false, |r| r.iter().any(|i| i == id))
        {
            Ok(())
        } else if let Some(error) = self.not_destroyed.as_mut().and_then(|r| r.remove(id)) {
            Err(error.to_string_error().into())
        } else {
            Err(Error::Internal(format!("Id {} not found.", id)))
        }
    }

    pub fn created_ids(&self) -> Option<impl Iterator<Item = &String>> {
        self.created.as_ref().map(|map| map.keys())
    }

    pub fn updated_ids(&self) -> Option<impl Iterator<Item = &String>> {
        self.updated.as_ref().map(|map| map.keys())
    }

    pub fn destroyed_ids(&self) -> Option<impl Iterator<Item = &String>> {
        self.destroyed.as_ref().map(|list| list.iter())
    }

    pub fn not_created_ids(&self) -> Option<impl Iterator<Item = &String>> {
        self.not_created.as_ref().map(|map| map.keys())
    }

    pub fn not_updated_ids(&self) -> Option<impl Iterator<Item = &String>> {
        self.not_updated.as_ref().map(|map| map.keys())
    }

    pub fn not_destroyed_ids(&self) -> Option<impl Iterator<Item = &String>> {
        self.not_destroyed.as_ref().map(|map| map.keys())
    }
}

impl<U: Display> SetError<U> {
    pub fn error(&self) -> &SetErrorType {
        &self.type_
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn properties(&self) -> Option<&[U]> {
        self.properties.as_deref()
    }

    pub fn to_string_error(&self) -> SetError<String> {
        SetError {
            type_: self.type_.clone(),
            description: self.description.as_ref().map(|s| s.to_string()),
            properties: self
                .properties
                .as_ref()
                .map(|s| s.iter().map(|s| s.to_string()).collect()),
        }
    }
}

impl<U: Display> Display for SetError<U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.type_.fmt(f)?;
        if let Some(description) = &self.description {
            write!(f, ": {}", description)?;
        }
        if let Some(properties) = &self.properties {
            write!(
                f,
                " (properties: {})",
                properties
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )?;
        }
        Ok(())
    }
}

impl Display for SetErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SetErrorType::Forbidden => write!(f, "Forbidden"),
            SetErrorType::OverQuota => write!(f, "OverQuota"),
            SetErrorType::TooLarge => write!(f, "TooLarge"),
            SetErrorType::RateLimit => write!(f, "RateLimit"),
            SetErrorType::NotFound => write!(f, "NotFound"),
            SetErrorType::InvalidPatch => write!(f, "InvalidPatch"),
            SetErrorType::WillDestroy => write!(f, "WillDestroy"),
            SetErrorType::InvalidProperties => write!(f, "InvalidProperties"),
            SetErrorType::Singleton => write!(f, "Singleton"),
            SetErrorType::MailboxHasChild => write!(f, "MailboxHasChild"),
            SetErrorType::MailboxHasEmail => write!(f, "MailboxHasEmail"),
            SetErrorType::BlobNotFound => write!(f, "BlobNotFound"),
            SetErrorType::TooManyKeywords => write!(f, "TooManyKeywords"),
            SetErrorType::TooManyMailboxes => write!(f, "TooManyMailboxes"),
            SetErrorType::ForbiddenFrom => write!(f, "ForbiddenFrom"),
            SetErrorType::InvalidEmail => write!(f, "InvalidEmail"),
            SetErrorType::TooManyRecipients => write!(f, "TooManyRecipients"),
            SetErrorType::NoRecipients => write!(f, "NoRecipients"),
            SetErrorType::InvalidRecipients => write!(f, "InvalidRecipients"),
            SetErrorType::ForbiddenMailFrom => write!(f, "ForbiddenMailFrom"),
            SetErrorType::ForbiddenToSend => write!(f, "ForbiddenToSend"),
            SetErrorType::CannotUnsend => write!(f, "CannotUnsend"),
        }
    }
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
