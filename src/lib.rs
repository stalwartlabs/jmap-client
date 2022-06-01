use crate::core::error::MethodError;
use crate::core::error::ProblemDetails;
use crate::core::set::SetError;
use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

pub mod blob;
pub mod client;
pub mod core;
pub mod email;
pub mod email_submission;
pub mod event_source;
pub mod identity;
pub mod mailbox;
pub mod push_subscription;
pub mod thread;
pub mod vacation_response;

#[cfg(feature = "websockets")]
pub mod client_ws;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum URI {
    #[serde(rename = "urn:ietf:params:jmap:core")]
    Core,
    #[serde(rename = "urn:ietf:params:jmap:mail")]
    Mail,
    #[serde(rename = "urn:ietf:params:jmap:submission")]
    Submission,
    #[serde(rename = "urn:ietf:params:jmap:vacationresponse")]
    VacationResponse,
    #[serde(rename = "urn:ietf:params:jmap:contacts")]
    Contacts,
    #[serde(rename = "urn:ietf:params:jmap:calendars")]
    Calendars,
    #[serde(rename = "urn:ietf:params:jmap:websocket")]
    WebSocket,
}

impl AsRef<str> for URI {
    fn as_ref(&self) -> &str {
        match self {
            URI::Core => "urn:ietf:params:jmap:core",
            URI::Mail => "urn:ietf:params:jmap:mail",
            URI::Submission => "urn:ietf:params:jmap:submission",
            URI::VacationResponse => "urn:ietf:params:jmap:vacationresponse",
            URI::Contacts => "urn:ietf:params:jmap:contacts",
            URI::Calendars => "urn:ietf:params:jmap:calendars",
            URI::WebSocket => "urn:ietf:params:jmap:websocket",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Method {
    #[serde(rename = "Core/echo")]
    Echo,
    #[serde(rename = "Blob/copy")]
    CopyBlob,
    #[serde(rename = "PushSubscription/get")]
    GetPushSubscription,
    #[serde(rename = "PushSubscription/set")]
    SetPushSubscription,
    #[serde(rename = "Mailbox/get")]
    GetMailbox,
    #[serde(rename = "Mailbox/changes")]
    ChangesMailbox,
    #[serde(rename = "Mailbox/query")]
    QueryMailbox,
    #[serde(rename = "Mailbox/queryChanges")]
    QueryChangesMailbox,
    #[serde(rename = "Mailbox/set")]
    SetMailbox,
    #[serde(rename = "Thread/get")]
    GetThread,
    #[serde(rename = "Thread/changes")]
    ChangesThread,
    #[serde(rename = "Email/get")]
    GetEmail,
    #[serde(rename = "Email/changes")]
    ChangesEmail,
    #[serde(rename = "Email/query")]
    QueryEmail,
    #[serde(rename = "Email/queryChanges")]
    QueryChangesEmail,
    #[serde(rename = "Email/set")]
    SetEmail,
    #[serde(rename = "Email/copy")]
    CopyEmail,
    #[serde(rename = "Email/import")]
    ImportEmail,
    #[serde(rename = "Email/parse")]
    ParseEmail,
    #[serde(rename = "SearchSnippet/get")]
    GetSearchSnippet,
    #[serde(rename = "Identity/get")]
    GetIdentity,
    #[serde(rename = "Identity/changes")]
    ChangesIdentity,
    #[serde(rename = "Identity/set")]
    SetIdentity,
    #[serde(rename = "EmailSubmission/get")]
    GetEmailSubmission,
    #[serde(rename = "EmailSubmission/changes")]
    ChangesEmailSubmission,
    #[serde(rename = "EmailSubmission/query")]
    QueryEmailSubmission,
    #[serde(rename = "EmailSubmission/queryChanges")]
    QueryChangesEmailSubmission,
    #[serde(rename = "EmailSubmission/set")]
    SetEmailSubmission,
    #[serde(rename = "VacationResponse/get")]
    GetVacationResponse,
    #[serde(rename = "VacationResponse/set")]
    SetVacationResponse,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub enum TypeState {
    Mailbox,
    Thread,
    Email,
    EmailDelivery,
    Identity,
    EmailSubmission,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StateChangeType {
    StateChange,
}

#[derive(Debug, Deserialize)]
pub struct StateChange {
    #[serde(rename = "@type")]
    pub type_: StateChangeType,
    pub changed: HashMap<String, HashMap<TypeState, String>>,
}

#[derive(Debug, Clone)]
pub struct Get;
#[derive(Debug, Clone)]
pub struct Set;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Transport(reqwest::Error),
    Parse(serde_json::Error),
    Internal(String),
    Problem(ProblemDetails),
    Server(String),
    Method(MethodError),
    Set(SetError<String>),
    #[cfg(feature = "websockets")]
    WebSocket(tokio_tungstenite::tungstenite::error::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Transport(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Parse(e)
    }
}

impl From<MethodError> for Error {
    fn from(e: MethodError) -> Self {
        Error::Method(e)
    }
}

impl From<ProblemDetails> for Error {
    fn from(e: ProblemDetails) -> Self {
        Error::Problem(e)
    }
}

impl From<SetError<String>> for Error {
    fn from(e: SetError<String>) -> Self {
        Error::Set(e)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Internal(s.to_string())
    }
}

#[cfg(feature = "websockets")]
impl From<tokio_tungstenite::tungstenite::error::Error> for Error {
    fn from(e: tokio_tungstenite::tungstenite::error::Error) -> Self {
        Error::WebSocket(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Transport(e) => write!(f, "Transport error: {}", e),
            Error::Parse(e) => write!(f, "Parse error: {}", e),
            Error::Internal(e) => write!(f, "Internal error: {}", e),
            Error::Problem(e) => write!(f, "Problem details: {}", e),
            Error::Server(e) => write!(f, "Server error: {}", e),
            Error::Method(e) => write!(f, "Method error: {}", e),
            Error::Set(e) => write!(f, "Set error: {}", e),
            #[cfg(feature = "websockets")]
            Error::WebSocket(e) => write!(f, "WebSocket error: {}", e),
        }
    }
}

impl Display for TypeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeState::Mailbox => write!(f, "Mailbox"),
            TypeState::Thread => write!(f, "Thread"),
            TypeState::Email => write!(f, "Email"),
            TypeState::EmailDelivery => write!(f, "EmailDelivery"),
            TypeState::Identity => write!(f, "Identity"),
            TypeState::EmailSubmission => write!(f, "EmailSubmission"),
        }
    }
}