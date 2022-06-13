pub mod get;
pub mod helpers;
pub mod query;
pub mod set;

use crate::core::set::{list_not_set, map_not_set, string_not_set};
use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::{
    core::{changes::ChangesObject, Object},
    Get, Set,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Principal<State = Get> {
    #[serde(skip)]
    _create_id: Option<usize>,

    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    ptype: Option<Type>,
    #[serde(skip_serializing_if = "string_not_set")]
    name: Option<String>,
    #[serde(skip_serializing_if = "string_not_set")]
    description: Option<String>,
    #[serde(skip_serializing_if = "string_not_set")]
    email: Option<String>,
    #[serde(skip_serializing_if = "string_not_set")]
    timezone: Option<String>,
    #[serde(skip_serializing_if = "list_not_set")]
    capabilities: Option<Vec<String>>,
    #[serde(skip_serializing_if = "list_not_set")]
    aliases: Option<Vec<String>>,
    #[serde(skip_serializing_if = "string_not_set")]
    secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dkim: Option<DKIM>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quota: Option<u32>,
    #[serde(skip_serializing_if = "string_not_set")]
    picture: Option<String>,
    #[serde(skip_serializing_if = "list_not_set")]
    members: Option<Vec<String>>,
    #[serde(skip_serializing_if = "map_not_set")]
    acl: Option<HashMap<String, Vec<ACL>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum Property {
    #[serde(rename = "id")]
    Id = 0,
    #[serde(rename = "type")]
    Type = 1,
    #[serde(rename = "name")]
    Name = 2,
    #[serde(rename = "description")]
    Description = 3,
    #[serde(rename = "email")]
    Email = 4,
    #[serde(rename = "timezone")]
    Timezone = 5,
    #[serde(rename = "capabilities")]
    Capabilities = 6,
    #[serde(rename = "aliases")]
    Aliases = 7,
    #[serde(rename = "secret")]
    Secret = 8,
    #[serde(rename = "dkim")]
    DKIM = 9,
    #[serde(rename = "quota")]
    Quota = 10,
    #[serde(rename = "picture")]
    Picture = 11,
    #[serde(rename = "members")]
    Members = 12,
    #[serde(rename = "acl")]
    ACL = 13,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum ACL {
    #[serde(rename = "read")]
    Read = 0,
    #[serde(rename = "modify")]
    Modify = 1,
    #[serde(rename = "delete")]
    Delete = 2,
    #[serde(rename = "readItems")]
    ReadItems = 3,
    #[serde(rename = "addItems")]
    AddItems = 4,
    #[serde(rename = "modifyItems")]
    ModifyItems = 5,
    #[serde(rename = "removeItems")]
    RemoveItems = 6,
    #[serde(rename = "createChild")]
    CreateChild = 7,
    #[serde(rename = "administer")]
    Administer = 8,
    #[serde(rename = "setSeen")]
    SetSeen = 9,
    #[serde(rename = "setKeywords")]
    SetKeywords = 10,
    #[serde(rename = "submit")]
    Submit = 11,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "individual")]
    Individual,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "resource")]
    Resource,
    #[serde(rename = "location")]
    Location,
    #[serde(rename = "domain")]
    Domain,
    #[serde(rename = "list")]
    List,
    #[serde(rename = "other")]
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DKIM {
    #[serde(rename = "dkimSelector")]
    pub dkim_selector: Option<String>,
    #[serde(rename = "dkimExpiration")]
    pub dkim_expiration: Option<i64>,
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Id => write!(f, "id"),
            Property::Type => write!(f, "type"),
            Property::Name => write!(f, "name"),
            Property::Description => write!(f, "description"),
            Property::Email => write!(f, "email"),
            Property::Timezone => write!(f, "timezone"),
            Property::Capabilities => write!(f, "capabilities"),
            Property::Aliases => write!(f, "aliases"),
            Property::Secret => write!(f, "secret"),
            Property::DKIM => write!(f, "dkim"),
            Property::Quota => write!(f, "quota"),
            Property::Picture => write!(f, "picture"),
            Property::Members => write!(f, "members"),
            Property::ACL => write!(f, "acl"),
        }
    }
}

impl Object for Principal<Set> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl Object for Principal<Get> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl ChangesObject for Principal<Set> {
    type ChangesResponse = ();
}

impl ChangesObject for Principal<Get> {
    type ChangesResponse = ();
}
