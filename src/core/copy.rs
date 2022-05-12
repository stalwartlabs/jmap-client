use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

use super::set::{Create, SetError};

#[derive(Debug, Clone, Serialize)]
pub struct CopyRequest<T: Create> {
    #[serde(rename = "fromAccountId")]
    from_account_id: String,

    #[serde(rename = "ifFromInState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    if_from_in_state: Option<String>,

    #[serde(rename = "accountId")]
    account_id: String,

    #[serde(rename = "ifInState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    if_in_state: Option<String>,

    #[serde(rename = "create")]
    create: HashMap<String, T>,

    #[serde(rename = "onSuccessDestroyOriginal")]
    on_success_destroy_original: bool,

    #[serde(rename = "destroyFromIfInState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    destroy_from_if_in_state: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CopyResponse<T, U: Display> {
    #[serde(rename = "fromAccountId")]
    from_account_id: String,

    #[serde(rename = "accountId")]
    account_id: String,

    #[serde(rename = "oldState")]
    old_state: Option<String>,

    #[serde(rename = "newState")]
    new_state: String,

    #[serde(rename = "created")]
    created: Option<HashMap<String, T>>,

    #[serde(rename = "notCreated")]
    not_created: Option<HashMap<String, SetError<U>>>,
}

impl<T: Create> CopyRequest<T> {
    pub fn new(from_account_id: String, account_id: String) -> Self {
        CopyRequest {
            from_account_id,
            if_from_in_state: None,
            account_id,
            if_in_state: None,
            create: HashMap::new(),
            on_success_destroy_original: false,
            destroy_from_if_in_state: None,
        }
    }

    pub fn account_id(&mut self, account_id: impl Into<String>) -> &mut Self {
        self.account_id = account_id.into();
        self
    }

    pub fn if_from_in_state(&mut self, if_from_in_state: impl Into<String>) -> &mut Self {
        self.if_from_in_state = Some(if_from_in_state.into());
        self
    }

    pub fn if_in_state(&mut self, if_in_state: impl Into<String>) -> &mut Self {
        self.if_in_state = Some(if_in_state.into());
        self
    }

    pub fn create(&mut self) -> &mut T {
        let create_id = self.create.len();
        let create_id_str = format!("c{}", create_id);
        self.create
            .insert(create_id_str.clone(), T::new(create_id.into()));
        self.create.get_mut(&create_id_str).unwrap()
    }

    pub fn on_success_destroy_original(&mut self, on_success_destroy_original: bool) -> &mut Self {
        self.on_success_destroy_original = on_success_destroy_original;
        self
    }

    pub fn destroy_from_if_in_state(
        &mut self,
        destroy_from_if_in_state: impl Into<String>,
    ) -> &mut Self {
        self.destroy_from_if_in_state = Some(destroy_from_if_in_state.into());
        self
    }
}

impl<T, U: Display> CopyResponse<T, U> {
    pub fn from_account_id(&self) -> &str {
        &self.from_account_id
    }

    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    pub fn old_state(&self) -> Option<&str> {
        self.old_state.as_deref()
    }

    pub fn new_state(&self) -> &str {
        &self.new_state
    }

    pub fn created(&self, id: &str) -> Option<&T> {
        self.created.as_ref().and_then(|created| created.get(id))
    }

    pub fn not_created(&self, id: &str) -> Option<&SetError<U>> {
        self.not_created
            .as_ref()
            .and_then(|not_created| not_created.get(id))
    }
}
