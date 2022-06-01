use std::collections::HashMap;

use crate::{core::get::GetObject, Get, Set};

use super::{Address, Delivered, DeliveryStatus, Displayed, EmailSubmission, UndoStatus};

impl EmailSubmission<Get> {
    pub fn id(&self) -> &str {
        self.id.as_ref().unwrap()
    }

    pub fn unwrap_id(self) -> String {
        self.id.unwrap()
    }

    pub fn identity_id(&self) -> &str {
        self.identity_id.as_ref().unwrap()
    }

    pub fn email_id(&self) -> &str {
        self.email_id.as_ref().unwrap()
    }

    pub fn thread_id(&self) -> &str {
        self.thread_id.as_ref().unwrap()
    }

    pub fn mail_from(&self) -> Option<&Address> {
        self.envelope.as_ref().map(|e| &e.mail_from)
    }

    pub fn rcpt_to(&self) -> Option<&[Address]> {
        self.envelope.as_ref().map(|e| e.rcpt_to.as_ref())
    }

    pub fn send_at(&self) -> i64 {
        self.send_at.as_ref().unwrap().timestamp()
    }

    pub fn undo_status(&self) -> &UndoStatus {
        self.undo_status.as_ref().unwrap()
    }

    pub fn delivery_status_email(&self, email: &str) -> Option<&DeliveryStatus> {
        self.delivery_status.as_ref().and_then(|ds| ds.get(email))
    }

    pub fn delivery_status(&self) -> Option<&HashMap<String, DeliveryStatus>> {
        self.delivery_status.as_ref()
    }

    pub fn dsn_blob_ids(&self) -> Option<&[String]> {
        self.dsn_blob_ids.as_deref()
    }

    pub fn mdn_blob_ids(&self) -> Option<&[String]> {
        self.mdn_blob_ids.as_deref()
    }
}

impl Address<Get> {
    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn parameter(&self, param: &str) -> Option<&str> {
        self.parameters.as_ref()?.get(param)?.as_deref()
    }

    pub fn has_parameter(&self, param: &str) -> bool {
        self.parameters
            .as_ref()
            .map(|ps| ps.contains_key(param))
            .unwrap_or(false)
    }
}

impl DeliveryStatus {
    #[cfg(feature = "debug")]
    pub fn new(smtp_reply: impl Into<String>, delivered: Delivered, displayed: Displayed) -> Self {
        Self {
            smtp_reply: smtp_reply.into(),
            delivered,
            displayed,
        }
    }

    pub fn smtp_reply(&self) -> &str {
        &self.smtp_reply
    }

    pub fn delivered(&self) -> &Delivered {
        &self.delivered
    }

    pub fn displayed(&self) -> &Displayed {
        &self.displayed
    }
}

impl GetObject for EmailSubmission<Set> {
    type GetArguments = ();
}

impl GetObject for EmailSubmission<Get> {
    type GetArguments = ();
}
