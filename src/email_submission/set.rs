use std::collections::HashMap;

use crate::{core::set::SetObject, Get, Set};

use super::{Address, EmailSubmission, Envelope, UndoStatus};

impl EmailSubmission<Set> {
    pub fn identity_id(&mut self, identity_id: impl Into<String>) -> &mut Self {
        self.identity_id = Some(identity_id.into());
        self
    }

    pub fn email_id(&mut self, email_id: impl Into<String>) -> &mut Self {
        self.email_id = Some(email_id.into());
        self
    }

    pub fn envelope<T, U>(&mut self, mail_from: U, rcpt_to: T) -> &mut Self
    where
        T: IntoIterator<Item = U>,
        U: Into<Address>,
    {
        self.envelope = Some(Envelope {
            mail_from: mail_from.into(),
            rcpt_to: rcpt_to.into_iter().map(|s| s.into()).collect(),
        });
        self
    }

    pub fn undo_status(&mut self, undo_status: UndoStatus) -> &mut Self {
        self.undo_status = Some(undo_status);
        self
    }
}

impl SetObject for EmailSubmission<Set> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        EmailSubmission {
            _create_id,
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

    fn create_id(&self) -> Option<String> {
        self._create_id.map(|id| format!("c{}", id))
    }
}

impl SetObject for EmailSubmission<Get> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        unimplemented!()
    }

    fn create_id(&self) -> Option<String> {
        None
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
