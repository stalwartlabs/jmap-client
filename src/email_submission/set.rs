use std::collections::HashMap;

use crate::Set;

use super::{Address, EmailSubmission, Envelope, UndoStatus};

impl EmailSubmission<Set> {
    pub fn identity_id(mut self, identity_id: String) -> Self {
        self.identity_id = Some(identity_id);
        self
    }

    pub fn email_id(mut self, email_id: String) -> Self {
        self.email_id = Some(email_id);
        self
    }

    pub fn envelope<T, U>(mut self, mail_from: U, rcpt_to: T) -> Self
    where
        T: Iterator<Item = U>,
        U: Into<Address>,
    {
        self.envelope = Some(Envelope {
            mail_from: mail_from.into(),
            rcpt_to: rcpt_to.map(|s| s.into()).collect(),
        });
        self
    }

    pub fn undo_status(mut self, undo_status: UndoStatus) -> Self {
        self.undo_status = Some(undo_status);
        self
    }
}

impl EmailSubmission {
    pub fn new() -> EmailSubmission<Set> {
        EmailSubmission {
            _state: Default::default(),
            id: None,
            identity_id: None,
            email_id: None,
            thread_id: None,
            envelope: None,
            send_at: None,
            undo_status: None,
            delivery_status: None,
            dsn_blob_ids: None,
            mdn_blob_ids: None,
        }
    }
}

impl Address {
    pub fn new(email: String) -> Address<Set> {
        Address {
            _state: Default::default(),
            email,
            parameters: None,
        }
    }
}

impl Address<Set> {
    pub fn parameter(mut self, parameter: String, value: Option<String>) -> Self {
        self.parameters
            .get_or_insert_with(HashMap::new)
            .insert(parameter, value);
        self
    }
}

impl From<String> for Address {
    fn from(email: String) -> Self {
        Address {
            _state: Default::default(),
            email,
            parameters: None,
        }
    }
}

impl From<&str> for Address {
    fn from(email: &str) -> Self {
        Address {
            _state: Default::default(),
            email: email.to_string(),
            parameters: None,
        }
    }
}
