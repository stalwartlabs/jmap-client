use std::collections::HashMap;

use crate::{core::get::GetObject, principal::ACL, Get, Set};

use super::{Mailbox, MailboxRights, Role};

impl Mailbox<Get> {
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn take_id(&mut self) -> String {
        self.id.take().unwrap_or_default()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn parent_id(&self) -> Option<&str> {
        self.parent_id.as_deref()
    }

    pub fn role(&self) -> Role {
        self.role.as_ref().cloned().unwrap_or(Role::None)
    }

    pub fn sort_order(&self) -> u32 {
        self.sort_order.as_ref().copied().unwrap_or(0)
    }

    pub fn total_emails(&self) -> usize {
        self.total_emails.as_ref().copied().unwrap_or(0)
    }

    pub fn unread_emails(&self) -> usize {
        self.unread_emails.as_ref().copied().unwrap_or(0)
    }

    pub fn total_threads(&self) -> usize {
        self.total_threads.as_ref().copied().unwrap_or(0)
    }

    pub fn unread_threads(&self) -> usize {
        self.unread_threads.as_ref().copied().unwrap_or(0)
    }

    pub fn is_subscribed(&self) -> bool {
        *self.is_subscribed.as_ref().unwrap_or(&false)
    }

    pub fn my_rights(&self) -> Option<&MailboxRights> {
        self.my_rights.as_ref()
    }

    pub fn acl(&self) -> Option<&HashMap<String, Vec<ACL>>> {
        self.acl.as_ref()
    }
}

impl MailboxRights {
    pub fn may_read_items(&self) -> bool {
        self.may_read_items
    }

    pub fn may_add_items(&self) -> bool {
        self.may_add_items
    }

    pub fn may_remove_items(&self) -> bool {
        self.may_remove_items
    }

    pub fn may_set_seen(&self) -> bool {
        self.may_set_seen
    }

    pub fn may_set_keywords(&self) -> bool {
        self.may_set_keywords
    }

    pub fn may_create_child(&self) -> bool {
        self.may_create_child
    }

    pub fn may_rename(&self) -> bool {
        self.may_rename
    }

    pub fn may_delete(&self) -> bool {
        self.may_delete
    }

    pub fn may_submit(&self) -> bool {
        self.may_submit
    }

    pub fn acl_list(&self) -> Vec<ACL> {
        let mut acl_list = Vec::new();
        for (is_set, acl) in [
            (self.may_read_items, ACL::ReadItems),
            (self.may_add_items, ACL::AddItems),
            (self.may_remove_items, ACL::RemoveItems),
            (self.may_set_seen, ACL::SetSeen),
            (self.may_set_keywords, ACL::SetKeywords),
            (self.may_create_child, ACL::CreateChild),
            (self.may_rename, ACL::Modify),
            (self.may_delete, ACL::Delete),
            (self.may_submit, ACL::Submit),
        ] {
            if is_set {
                acl_list.push(acl);
            }
        }
        acl_list
    }
}

impl GetObject for Mailbox<Set> {
    type GetArguments = ();
}

impl GetObject for Mailbox<Get> {
    type GetArguments = ();
}
