use crate::Get;

use super::{Mailbox, Role};

impl Mailbox<Get> {
    pub fn id(&self) -> &str {
        self.id.as_ref().unwrap()
    }

    pub fn unwrap_id(self) -> String {
        self.id.unwrap()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref().unwrap()
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

    pub fn may_read_items(&self) -> bool {
        self.my_rights.as_ref().unwrap().may_read_items
    }

    pub fn may_add_items(&self) -> bool {
        self.my_rights.as_ref().unwrap().may_add_items
    }

    pub fn may_remove_items(&self) -> bool {
        self.my_rights.as_ref().unwrap().may_remove_items
    }

    pub fn may_set_seen(&self) -> bool {
        self.my_rights.as_ref().unwrap().may_set_seen
    }

    pub fn may_set_keywords(&self) -> bool {
        self.my_rights.as_ref().unwrap().may_set_keywords
    }

    pub fn may_create_child(&self) -> bool {
        self.my_rights.as_ref().unwrap().may_create_child
    }

    pub fn may_rename(&self) -> bool {
        self.my_rights.as_ref().unwrap().may_rename
    }

    pub fn may_delete(&self) -> bool {
        self.my_rights.as_ref().unwrap().may_delete
    }

    pub fn may_submit(&self) -> bool {
        self.my_rights.as_ref().unwrap().may_submit
    }
}
