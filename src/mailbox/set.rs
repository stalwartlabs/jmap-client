use crate::Set;

use super::{Mailbox, Role};

impl Mailbox<Set> {
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn parent_id(mut self, parent_id: Option<String>) -> Self {
        self.parent_id = parent_id;
        self
    }

    pub fn role(mut self, role: Role) -> Self {
        if !matches!(role, Role::None) {
            self.role = Some(role);
        } else {
            self.role = None;
        }
        self
    }

    pub fn sort_order(mut self, sort_order: u32) -> Self {
        self.sort_order = sort_order.into();
        self
    }
}

pub fn role_not_set(role: &Option<Role>) -> bool {
    matches!(role, Some(Role::None))
}

impl Mailbox {
    pub fn new() -> Mailbox<Set> {
        Mailbox {
            _state: Default::default(),
            id: None,
            name: None,
            parent_id: "".to_string().into(),
            role: Role::None.into(),
            sort_order: None,
            total_emails: None,
            unread_emails: None,
            total_threads: None,
            unread_threads: None,
            my_rights: None,
            is_subscribed: None,
        }
    }
}
