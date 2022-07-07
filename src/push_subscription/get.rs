use crate::{core::get::GetObject, Get, Set, TypeState};

use super::{Keys, PushSubscription};

impl PushSubscription<Get> {
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn unwrap_id(self) -> String {
        self.id.unwrap_or_default()
    }

    pub fn device_client_id(&self) -> Option<&str> {
        self.device_client_id.as_deref()
    }

    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    pub fn keys(&self) -> Option<&Keys> {
        self.keys.as_ref()
    }

    pub fn verification_code(&self) -> Option<&str> {
        self.verification_code.as_deref()
    }

    pub fn expires(&self) -> Option<i64> {
        self.expires.map(|v| v.timestamp())
    }

    pub fn types(&self) -> Option<&[TypeState]> {
        self.types.as_deref()
    }
}

impl Keys {
    pub fn p256dh(&self) -> Option<Vec<u8>> {
        base64::decode_config(&self.p256dh, base64::URL_SAFE).ok()
    }

    pub fn auth(&self) -> Option<Vec<u8>> {
        base64::decode_config(&self.auth, base64::URL_SAFE).ok()
    }
}

impl GetObject for PushSubscription<Set> {
    type GetArguments = ();
}

impl GetObject for PushSubscription<Get> {
    type GetArguments = ();
}
