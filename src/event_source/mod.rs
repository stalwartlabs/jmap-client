pub mod parser;
pub mod stream;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{core::session::URLParser, TypeState};

pub enum URLParameter {
    Types,
    CloseAfter,
    Ping,
}

impl URLParser for URLParameter {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "types" => Some(URLParameter::Types),
            "closeafter" => Some(URLParameter::CloseAfter),
            "ping" => Some(URLParameter::Ping),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Changes {
    id: Option<String>,
    changes: HashMap<String, HashMap<TypeState, String>>,
}

impl Changes {
    pub fn new(id: Option<String>, changes: HashMap<String, HashMap<TypeState, String>>) -> Self {
        Self { id, changes }
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn account_changes(&mut self, account_id: &str) -> Option<HashMap<TypeState, String>> {
        self.changes.remove(account_id)
    }

    pub fn changed_accounts(&self) -> impl Iterator<Item = &String> {
        self.changes.keys()
    }

    pub fn changes(&self, account_id: &str) -> Option<impl Iterator<Item = (&TypeState, &String)>> {
        self.changes.get(account_id).map(|changes| changes.iter())
    }

    pub fn has_type(&self, type_: TypeState) -> bool {
        self.changes
            .values()
            .any(|changes| changes.contains_key(&type_))
    }

    pub fn into_inner(self) -> HashMap<String, HashMap<TypeState, String>> {
        self.changes
    }

    pub fn is_empty(&self) -> bool {
        !self.changes.values().any(|changes| !changes.is_empty())
    }
}
