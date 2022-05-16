use crate::{Get, TypeState};

use super::{Keys, PushSubscription};

impl PushSubscription<Get> {
    pub fn id(&self) -> &str {
        self.id.as_ref().unwrap()
    }

    pub fn unwrap_id(self) -> String {
        self.id.unwrap()
    }

    pub fn device_client_id(&self) -> &str {
        self.device_client_id.as_ref().unwrap()
    }

    pub fn url(&self) -> &str {
        self.url.as_ref().unwrap()
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
