use crate::{
    core::set::{from_timestamp, Create},
    Set, TypeState,
};

use super::{Keys, PushSubscription};

impl PushSubscription<Set> {
    pub fn device_client_id(&mut self, device_client_id: impl Into<String>) -> &mut Self {
        self.device_client_id = Some(device_client_id.into());
        self
    }

    pub fn url(&mut self, url: impl Into<String>) -> &mut Self {
        self.url = Some(url.into());
        self
    }

    pub fn verification_code(&mut self, verification_code: impl Into<String>) -> &mut Self {
        self.verification_code = Some(verification_code.into());
        self
    }

    pub fn keys(&mut self, keys: Keys) -> &mut Self {
        self.keys = Some(keys);
        self
    }

    pub fn expires(&mut self, expires: i64) -> &mut Self {
        self.expires = Some(from_timestamp(expires));
        self
    }

    pub fn types(&mut self, types: Option<impl IntoIterator<Item = TypeState>>) -> &mut Self {
        self.types = types.map(|s| s.into_iter().collect());
        self
    }
}

impl Create for PushSubscription<Set> {
    fn new(_create_id: Option<usize>) -> Self {
        PushSubscription {
            _create_id,
            _state: Default::default(),
            id: None,
            device_client_id: None,
            url: None,
            keys: None,
            verification_code: None,
            expires: None,
            types: Vec::with_capacity(0).into(),
        }
    }

    fn create_id(&self) -> Option<String> {
        self._create_id.map(|id| format!("c{}", id))
    }
}

impl Keys {
    pub fn new(p256dh: &[u8], auth: &[u8]) -> Self {
        Keys {
            p256dh: base64::encode_config(&p256dh, base64::URL_SAFE),
            auth: base64::encode_config(&auth, base64::URL_SAFE),
        }
    }
}
