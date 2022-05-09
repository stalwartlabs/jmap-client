pub mod get;
pub mod set;

use crate::core::set::list_not_set;
use crate::{email::EmailAddress, Get};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity<State = Get> {
    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "replyTo")]
    #[serde(skip_serializing_if = "list_not_set")]
    pub reply_to: Option<Vec<EmailAddress>>,

    #[serde(rename = "bcc")]
    #[serde(skip_serializing_if = "list_not_set")]
    pub bcc: Option<Vec<EmailAddress>>,

    #[serde(rename = "textSignature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_signature: Option<String>,

    #[serde(rename = "htmlSignature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_signature: Option<String>,

    #[serde(rename = "mayDelete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub may_delete: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdentityProperty {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "replyTo")]
    ReplyTo,
    #[serde(rename = "bcc")]
    Bcc,
    #[serde(rename = "textSignature")]
    TextSignature,
    #[serde(rename = "htmlSignature")]
    HtmlSignature,
    #[serde(rename = "mayDelete")]
    MayDelete,
}
