use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::Method;

use super::{request::ResultReference, RequestParams, Type};

#[derive(Debug, Clone, Serialize)]
pub struct GetRequest<T: Display + Type, A: Default> {
    #[serde(skip)]
    method: (Method, usize),

    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ids: Option<Vec<String>>,

    #[serde(rename = "#ids")]
    #[serde(skip_deserializing)]
    #[serde(skip_serializing_if = "Option::is_none")]
    ids_ref: Option<ResultReference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<Vec<T>>,

    #[serde(flatten)]
    arguments: A,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetResponse<T> {
    #[serde(rename = "accountId")]
    account_id: Option<String>,

    state: String,

    list: Vec<T>,

    #[serde(rename = "notFound")]
    not_found: Vec<String>,
}

impl<T: Display + Type, A: Default> GetRequest<T, A> {
    pub fn new(params: RequestParams) -> Self {
        GetRequest {
            account_id: if T::requires_account_id() {
                params.account_id.into()
            } else {
                None
            },
            method: (params.method, params.call_id),
            ids: None,
            ids_ref: None,
            properties: None,
            arguments: A::default(),
        }
    }

    pub fn account_id(&mut self, account_id: impl Into<String>) -> &mut Self {
        if T::requires_account_id() {
            self.account_id = Some(account_id.into());
        }
        self
    }

    pub fn ids<U, V>(&mut self, ids: U) -> &mut Self
    where
        U: IntoIterator<Item = V>,
        V: Into<String>,
    {
        self.ids = Some(ids.into_iter().map(|v| v.into()).collect());
        self.ids_ref = None;
        self
    }

    pub fn ids_ref(&mut self, reference: ResultReference) -> &mut Self {
        self.ids_ref = reference.into();
        self.ids = None;
        self
    }

    pub fn properties(&mut self, properties: impl IntoIterator<Item = T>) -> &mut Self {
        self.properties = Some(properties.into_iter().collect());
        self
    }

    pub fn arguments(&mut self) -> &mut A {
        &mut self.arguments
    }

    pub fn result_reference(&self, property: T) -> ResultReference {
        ResultReference::new(
            self.method.0,
            self.method.1,
            format!("/list/*/{}", property),
        )
    }
}

impl<T> GetResponse<T> {
    pub fn account_id(&self) -> &str {
        self.account_id.as_ref().unwrap()
    }

    pub fn state(&self) -> &str {
        &self.state
    }

    pub fn list(&self) -> &[T] {
        &self.list
    }

    pub fn not_found(&self) -> &[String] {
        &self.not_found
    }

    pub fn unwrap_list(&mut self) -> Vec<T> {
        std::mem::take(&mut self.list)
    }
    pub fn pop(&mut self) -> Option<T> {
        self.list.pop()
    }

    pub fn unwrap_not_found(&mut self) -> Vec<String> {
        std::mem::take(&mut self.not_found)
    }
}
