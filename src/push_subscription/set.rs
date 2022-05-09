use crate::{core::set::from_timestamp, Object, Set};

use super::{Keys, PushSubscription};

impl PushSubscription<Set> {
    pub fn device_client_id(mut self, device_client_id: String) -> Self {
        self.device_client_id = Some(device_client_id);
        self
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    pub fn verification_code(mut self, verification_code: String) -> Self {
        self.verification_code = Some(verification_code);
        self
    }

    pub fn keys(mut self, keys: Keys) -> Self {
        self.keys = Some(keys);
        self
    }

    pub fn expires(mut self, expires: i64) -> Self {
        self.expires = Some(from_timestamp(expires));
        self
    }

    pub fn types(mut self, types: Option<impl Iterator<Item = Object>>) -> Self {
        self.types = types.map(|s| s.collect());
        self
    }
}

impl PushSubscription {
    pub fn new() -> PushSubscription<Set> {
        PushSubscription {
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
}

impl Keys {
    pub fn new(p256dh: &[u8], auth: &[u8]) -> Self {
        Keys {
            p256dh: base64::encode_config(&p256dh, base64::URL_SAFE),
            auth: base64::encode_config(&auth, base64::URL_SAFE),
        }
    }

    pub fn generate() -> Option<Self> {
        let (p256dh, auth) = ece::generate_keypair_and_auth_secret().ok()?;
        Self::new(&p256dh.pub_as_raw().ok()?, &auth).into()
    }
}
