use crate::{core::set::Create, email::EmailAddress, Set};

use super::Identity;

impl Identity<Set> {
    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn email(&mut self, email: String) -> &mut Self {
        self.email = Some(email);
        self
    }

    pub fn bcc<T, U>(&mut self, bcc: Option<T>) -> &mut Self
    where
        T: Iterator<Item = U>,
        U: Into<EmailAddress>,
    {
        self.bcc = bcc.map(|s| s.map(|s| s.into()).collect());
        self
    }

    pub fn reply_to<T, U>(&mut self, reply_to: Option<T>) -> &mut Self
    where
        T: Iterator<Item = U>,
        U: Into<EmailAddress>,
    {
        self.reply_to = reply_to.map(|s| s.map(|s| s.into()).collect());
        self
    }

    pub fn text_signature(&mut self, text_signature: String) -> &mut Self {
        self.text_signature = Some(text_signature);
        self
    }

    pub fn html_signature(&mut self, html_signature: String) -> &mut Self {
        self.html_signature = Some(html_signature);
        self
    }
}

impl Create for Identity<Set> {
    fn new(_create_id: Option<usize>) -> Self {
        Identity {
            _create_id,
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

    fn create_id(&self) -> Option<String> {
        self._create_id.map(|id| format!("c{}", id))
    }
}
