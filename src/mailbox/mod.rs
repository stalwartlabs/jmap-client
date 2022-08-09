pub mod get;
#[cfg(feature = "async")]
pub mod helpers;
#[cfg(feature = "blocking")]
pub mod helpers_blocking;
pub mod query;
pub mod set;

use crate::core::changes::ChangesObject;
use crate::core::set::{map_not_set, string_not_set};
use crate::core::Object;
use crate::mailbox::set::role_not_set;
use crate::principal::ACL;
use crate::{Get, Set};
use ahash::AHashMap;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Default)]
pub struct SetArguments {
    #[serde(rename = "onDestroyRemoveEmails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    on_destroy_remove_emails: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct QueryArguments {
    #[serde(rename = "sortAsTree")]
    sort_as_tree: bool,
    #[serde(rename = "filterAsTree")]
    filter_as_tree: bool,
}

#[derive(Debug, Deserialize, Default)]
pub struct ChangesResponse {
    #[serde(rename = "updatedProperties")]
    updated_properties: Option<Vec<Property>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mailbox<State = Get> {
    #[serde(skip)]
    _create_id: Option<usize>,

    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(rename = "parentId")]
    #[serde(skip_serializing_if = "string_not_set")]
    parent_id: Option<String>,

    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "role_not_set")]
    role: Option<Role>,

    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_order: Option<u32>,

    #[serde(rename = "totalEmails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    total_emails: Option<usize>,

    #[serde(rename = "unreadEmails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    unread_emails: Option<usize>,

    #[serde(rename = "totalThreads")]
    #[serde(skip_serializing_if = "Option::is_none")]
    total_threads: Option<usize>,

    #[serde(rename = "unreadThreads")]
    #[serde(skip_serializing_if = "Option::is_none")]
    unread_threads: Option<usize>,

    #[serde(rename = "myRights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    my_rights: Option<MailboxRights>,

    #[serde(rename = "isSubscribed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    is_subscribed: Option<bool>,

    #[serde(skip_serializing_if = "map_not_set")]
    acl: Option<AHashMap<String, Vec<ACL>>>,

    #[serde(flatten)]
    #[serde(skip_deserializing)]
    #[serde(skip_serializing_if = "Option::is_none")]
    acl_patch: Option<AHashMap<String, ACLPatch>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum ACLPatch {
    Replace(Vec<ACL>),
    Set(bool),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    #[serde(rename = "archive", alias = "ARCHIVE")]
    Archive,
    #[serde(rename = "drafts", alias = "DRAFTS")]
    Drafts,
    #[serde(rename = "important", alias = "IMPORTANT")]
    Important,
    #[serde(rename = "inbox", alias = "INBOX")]
    Inbox,
    #[serde(rename = "junk", alias = "JUNK")]
    Junk,
    #[serde(rename = "sent", alias = "SENT")]
    Sent,
    #[serde(rename = "trash", alias = "TRASH")]
    Trash,
    None,
}

impl Default for Role {
    fn default() -> Self {
        Role::None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailboxRights {
    #[serde(rename = "mayReadItems")]
    may_read_items: bool,

    #[serde(rename = "mayAddItems")]
    may_add_items: bool,

    #[serde(rename = "mayRemoveItems")]
    may_remove_items: bool,

    #[serde(rename = "maySetSeen")]
    may_set_seen: bool,

    #[serde(rename = "maySetKeywords")]
    may_set_keywords: bool,

    #[serde(rename = "mayCreateChild")]
    may_create_child: bool,

    #[serde(rename = "mayRename")]
    may_rename: bool,

    #[serde(rename = "mayDelete")]
    may_delete: bool,

    #[serde(rename = "maySubmit")]
    may_submit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum Property {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "parentId")]
    ParentId,
    #[serde(rename = "role")]
    Role,
    #[serde(rename = "sortOrder")]
    SortOrder,
    #[serde(rename = "totalEmails")]
    TotalEmails,
    #[serde(rename = "unreadEmails")]
    UnreadEmails,
    #[serde(rename = "totalThreads")]
    TotalThreads,
    #[serde(rename = "unreadThreads")]
    UnreadThreads,
    #[serde(rename = "myRights")]
    MyRights,
    #[serde(rename = "isSubscribed")]
    IsSubscribed,
    #[serde(rename = "acl")]
    ACL,
}

impl Property {
    pub fn is_count(&self) -> bool {
        matches!(
            self,
            Property::TotalEmails
                | Property::UnreadEmails
                | Property::TotalThreads
                | Property::UnreadThreads
        )
    }
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Id => write!(f, "id"),
            Property::Name => write!(f, "name"),
            Property::ParentId => write!(f, "parentId"),
            Property::Role => write!(f, "role"),
            Property::SortOrder => write!(f, "sortOrder"),
            Property::TotalEmails => write!(f, "totalEmails"),
            Property::UnreadEmails => write!(f, "unreadEmails"),
            Property::TotalThreads => write!(f, "totalThreads"),
            Property::UnreadThreads => write!(f, "unreadThreads"),
            Property::MyRights => write!(f, "myRights"),
            Property::IsSubscribed => write!(f, "isSubscribed"),
            Property::ACL => write!(f, "acl"),
        }
    }
}

impl ChangesResponse {
    pub fn updated_properties(&self) -> Option<&[Property]> {
        self.updated_properties.as_deref()
    }
}

impl Object for Mailbox<Set> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl Object for Mailbox<Get> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl ChangesObject for Mailbox<Set> {
    type ChangesResponse = ChangesResponse;
}

impl ChangesObject for Mailbox<Get> {
    type ChangesResponse = ChangesResponse;
}
