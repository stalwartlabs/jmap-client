use crate::{email::EmailAddress, Set};

use super::Identity;

impl Identity<Set> {
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    pub fn bcc<T, U>(mut self, bcc: Option<T>) -> Self
    where
        T: Iterator<Item = U>,
        U: Into<EmailAddress>,
    {
        self.bcc = bcc.map(|s| s.map(|s| s.into()).collect());
        self
    }

    pub fn reply_to<T, U>(mut self, reply_to: Option<T>) -> Self
    where
        T: Iterator<Item = U>,
        U: Into<EmailAddress>,
    {
        self.reply_to = reply_to.map(|s| s.map(|s| s.into()).collect());
        self
    }

    pub fn text_signature(mut self, text_signature: String) -> Self {
        self.text_signature = Some(text_signature);
        self
    }

    pub fn html_signature(mut self, html_signature: String) -> Self {
        self.html_signature = Some(html_signature);
        self
    }
}

impl Identity {
    pub fn new() -> Identity<Set> {
        Identity {
            _state: Default::default(),
            id: None,
            name: None,
            email: None,
            reply_to: Vec::with_capacity(0).into(),
            bcc: Vec::with_capacity(0).into(),
            text_signature: None,
            html_signature: None,
            may_delete: None,
        }
    }
}
