use crate::{core::get::GetObject, email::EmailAddress, Get, Set};

use super::Identity;

impl Identity<Get> {
    pub fn id(&self) -> &str {
        self.id.as_ref().unwrap()
    }

    pub fn unwrap_id(self) -> String {
        self.id.unwrap()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn email(&self) -> &str {
        self.email.as_ref().unwrap()
    }

    pub fn reply_to(&self) -> Option<&[EmailAddress]> {
        self.reply_to.as_deref()
    }

    pub fn bcc(&self) -> Option<&[EmailAddress]> {
        self.bcc.as_deref()
    }

    pub fn text_signature(&self) -> Option<&str> {
        self.text_signature.as_deref()
    }

    pub fn html_signature(&self) -> Option<&str> {
        self.html_signature.as_deref()
    }

    pub fn may_delete(&self) -> bool {
        self.may_delete.unwrap_or(false)
    }
}

impl GetObject for Identity<Set> {
    type GetArguments = ();
}

impl GetObject for Identity<Get> {
    type GetArguments = ();
}
