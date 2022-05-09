use crate::{core::set::from_timestamp, Set};

use super::VacationResponse;

impl VacationResponse<Set> {
    pub fn is_enabled(mut self, is_enabled: bool) -> Self {
        self.is_enabled = Some(is_enabled);
        self
    }

    pub fn from_date(mut self, from_date: Option<i64>) -> Self {
        self.from_date = from_date.map(from_timestamp);
        self
    }

    pub fn to_date(mut self, to_date: Option<i64>) -> Self {
        self.to_date = to_date.map(from_timestamp);
        self
    }

    pub fn subject(mut self, subject: Option<String>) -> Self {
        self.subject = subject;
        self
    }

    pub fn text_body(mut self, text_body: Option<String>) -> Self {
        self.text_body = text_body;
        self
    }

    pub fn html_body(mut self, html_body: Option<String>) -> Self {
        self.html_body = html_body;
        self
    }
}

impl VacationResponse {
    pub fn new() -> VacationResponse<Set> {
        VacationResponse {
            _state: Default::default(),
            id: None,
            is_enabled: None,
            from_date: from_timestamp(0).into(),
            to_date: from_timestamp(0).into(),
            subject: "".to_string().into(),
            text_body: "".to_string().into(),
            html_body: "".to_string().into(),
        }
    }
}
