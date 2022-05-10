use serde::Serialize;

use crate::core::query::{self};

use super::Role;

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Filter {
    ParentId {
        #[serde(rename = "parentId")]
        value: Option<String>,
    },
    Name {
        #[serde(rename = "name")]
        value: String,
    },
    Role {
        #[serde(rename = "role")]
        value: Option<Role>,
    },
    HasAnyRole {
        #[serde(rename = "hasAnyRole")]
        value: bool,
    },
    IsSubscribed {
        #[serde(rename = "isSubscribed")]
        value: bool,
    },
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "property")]
pub enum Comparator {
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "sortOrder")]
    SortOrder,
    #[serde(rename = "parentId")]
    ParentId,
}

impl Filter {
    pub fn parent_id(value: Option<String>) -> Self {
        Filter::ParentId { value }
    }

    pub fn name(value: String) -> Self {
        Filter::Name { value }
    }

    pub fn role(value: Role) -> Self {
        Filter::Role {
            value: if !matches!(value, Role::None) {
                value.into()
            } else {
                None
            },
        }
    }

    pub fn has_any_role(value: bool) -> Self {
        Filter::HasAnyRole { value }
    }

    pub fn is_subscribed(value: bool) -> Self {
        Filter::IsSubscribed { value }
    }
}

impl Comparator {
    pub fn name() -> query::Comparator<Comparator> {
        query::Comparator::new(Comparator::Name)
    }

    pub fn sort_order() -> query::Comparator<Comparator> {
        query::Comparator::new(Comparator::SortOrder)
    }

    pub fn parent_id() -> query::Comparator<Comparator> {
        query::Comparator::new(Comparator::ParentId)
    }
}
