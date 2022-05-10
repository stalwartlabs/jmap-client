use std::collections::HashMap;

use serde::Serialize;

use super::{BodyProperty, Email, Property};

#[derive(Debug, Clone, Serialize)]
pub struct EmailParseRequest {
    #[serde(rename = "accountId")]
    account_id: String,

    #[serde(rename = "blobIds")]
    blob_ids: Vec<String>,

    #[serde(rename = "properties")]
    properties: Vec<Property>,

    #[serde(rename = "bodyProperties")]
    body_properties: Vec<BodyProperty>,

    #[serde(rename = "fetchTextBodyValues")]
    fetch_text_body_values: bool,

    #[serde(rename = "fetchHTMLBodyValues")]
    fetch_html_body_values: bool,

    #[serde(rename = "fetchAllBodyValues")]
    fetch_all_body_values: bool,

    #[serde(rename = "maxBodyValueBytes")]
    max_body_value_bytes: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct EmailParseResponse {
    #[serde(rename = "accountId")]
    account_id: String,

    #[serde(rename = "parsed")]
    parsed: Option<HashMap<String, Email>>,

    #[serde(rename = "notParsable")]
    not_parsable: Option<Vec<String>>,

    #[serde(rename = "notFound")]
    not_found: Option<Vec<String>>,
}

impl EmailParseRequest {
    pub fn new(account_id: String) -> Self {
        EmailParseRequest {
            account_id,
            blob_ids: Vec::new(),
            properties: Vec::new(),
            body_properties: Vec::new(),
            fetch_text_body_values: false,
            fetch_html_body_values: false,
            fetch_all_body_values: false,
            max_body_value_bytes: 0,
        }
    }

    pub fn blob_ids<U, V>(&mut self, blob_ids: U) -> &mut Self
    where
        U: IntoIterator<Item = V>,
        V: Into<String>,
    {
        self.blob_ids = blob_ids.into_iter().map(|v| v.into()).collect();
        self
    }

    pub fn properties(&mut self, properties: impl IntoIterator<Item = Property>) -> &mut Self {
        self.properties = properties.into_iter().collect();
        self
    }

    pub fn body_properties(
        &mut self,
        body_properties: impl IntoIterator<Item = BodyProperty>,
    ) -> &mut Self {
        self.body_properties = body_properties.into_iter().collect();
        self
    }

    pub fn fetch_text_body_values(&mut self, fetch_text_body_values: bool) -> &mut Self {
        self.fetch_text_body_values = fetch_text_body_values;
        self
    }

    pub fn fetch_html_body_values(&mut self, fetch_html_body_values: bool) -> &mut Self {
        self.fetch_html_body_values = fetch_html_body_values;
        self
    }

    pub fn fetch_all_body_values(&mut self, fetch_all_body_values: bool) -> &mut Self {
        self.fetch_all_body_values = fetch_all_body_values;
        self
    }

    pub fn max_body_value_bytes(&mut self, max_body_value_bytes: usize) -> &mut Self {
        self.max_body_value_bytes = max_body_value_bytes;
        self
    }
}

impl EmailParseResponse {
    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    pub fn parsed(&self) -> Option<impl Iterator<Item = &String>> {
        self.parsed.as_ref().map(|map| map.keys())
    }

    pub fn parsed_details(&self, id: &str) -> Option<&Email> {
        self.parsed.as_ref().and_then(|map| map.get(id))
    }

    pub fn not_parsable(&self) -> Option<&[String]> {
        self.not_parsable.as_deref()
    }

    pub fn not_found(&self) -> Option<&[String]> {
        self.not_found.as_deref()
    }
}
