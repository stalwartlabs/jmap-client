pub mod get;
pub mod set;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::core::set::list_not_set;
use crate::{Get, Object};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushSubscription<State = Get> {
    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(rename = "deviceClientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    device_client_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,

    #[serde(rename = "keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    keys: Option<Keys>,

    #[serde(rename = "verificationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    verification_code: Option<String>,

    #[serde(rename = "expires")]
    #[serde(skip_serializing_if = "Option::is_none")]
    expires: Option<DateTime<Utc>>,

    #[serde(rename = "types")]
    #[serde(skip_serializing_if = "list_not_set")]
    types: Option<Vec<Object>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Property {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "deviceClientId")]
    DeviceClientId,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "keys")]
    Keys,
    #[serde(rename = "verificationCode")]
    VerificationCode,
    #[serde(rename = "expires")]
    Expires,
    #[serde(rename = "types")]
    Types,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keys {
    p256dh: String,
    auth: String,
}
