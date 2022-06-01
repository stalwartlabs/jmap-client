pub mod get;
pub mod helpers;
pub mod query;
pub mod set;

use std::{collections::HashMap, fmt::Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    core::{changes::ChangesObject, Object},
    email::Email,
    Get, Set,
};

#[derive(Debug, Clone, Serialize, Default)]
pub struct SetArguments {
    #[serde(rename = "onSuccessUpdateEmail")]
    on_success_update_email: Option<HashMap<String, Email>>,
    #[serde(rename = "onSuccessDestroyEmail")]
    on_success_destroy_email: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailSubmission<State = Get> {
    #[serde(skip)]
    _create_id: Option<usize>,

    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(rename = "identityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_id: Option<String>,

    #[serde(rename = "emailId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    email_id: Option<String>,

    #[serde(rename = "threadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    thread_id: Option<String>,

    #[serde(rename = "envelope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    envelope: Option<Envelope>,

    #[serde(rename = "sendAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    send_at: Option<DateTime<Utc>>,

    #[serde(rename = "undoStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    undo_status: Option<UndoStatus>,

    #[serde(rename = "deliveryStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_status: Option<HashMap<String, DeliveryStatus>>,

    #[serde(rename = "dsnBlobIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dsn_blob_ids: Option<Vec<String>>,

    #[serde(rename = "mdnBlobIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    mdn_blob_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope {
    #[serde(rename = "mailFrom")]
    mail_from: Address,

    #[serde(rename = "rcptTo")]
    rcpt_to: Vec<Address>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address<State = Get> {
    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    email: String,
    parameters: Option<HashMap<String, Option<String>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UndoStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "final")]
    Final,
    #[serde(rename = "canceled")]
    Canceled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryStatus {
    #[serde(rename = "smtpReply")]
    smtp_reply: String,

    #[serde(rename = "delivered")]
    delivered: Delivered,

    #[serde(rename = "displayed")]
    displayed: Displayed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Delivered {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Displayed {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "yes")]
    Yes,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum Property {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "identityId")]
    IdentityId,
    #[serde(rename = "emailId")]
    EmailId,
    #[serde(rename = "threadId")]
    ThreadId,
    #[serde(rename = "envelope")]
    Envelope,
    #[serde(rename = "sendAt")]
    SendAt,
    #[serde(rename = "undoStatus")]
    UndoStatus,
    #[serde(rename = "deliveryStatus")]
    DeliveryStatus,
    #[serde(rename = "dsnBlobIds")]
    DsnBlobIds,
    #[serde(rename = "mdnBlobIds")]
    MdnBlobIds,
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Id => write!(f, "id"),
            Property::IdentityId => write!(f, "identityId"),
            Property::EmailId => write!(f, "emailId"),
            Property::ThreadId => write!(f, "threadId"),
            Property::Envelope => write!(f, "envelope"),
            Property::SendAt => write!(f, "sendAt"),
            Property::UndoStatus => write!(f, "undoStatus"),
            Property::DeliveryStatus => write!(f, "deliveryStatus"),
            Property::DsnBlobIds => write!(f, "dsnBlobIds"),
            Property::MdnBlobIds => write!(f, "mdnBlobIds"),
        }
    }
}

impl Object for EmailSubmission<Set> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl Object for EmailSubmission<Get> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl ChangesObject for EmailSubmission<Set> {
    type ChangesResponse = ();
}

impl ChangesObject for EmailSubmission<Get> {
    type ChangesResponse = ();
}
