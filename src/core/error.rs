use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProblemDetails {
    #[serde(rename = "type")]
    p_type: ProblemType,
    status: Option<u32>,
    title: Option<String>,
    detail: Option<String>,
    limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub enum ProblemType {
    #[serde(rename = "urn:ietf:params:jmap:error:unknownCapability")]
    UnknownCapability,
    #[serde(rename = "urn:ietf:params:jmap:error:notJSON")]
    NotJSON,
    #[serde(rename = "urn:ietf:params:jmap:error:notRequest")]
    NotRequest,
    #[serde(rename = "urn:ietf:params:jmap:error:limit")]
    Limit,
}

#[derive(Debug, Deserialize)]
pub struct MethodError {
    #[serde(rename = "type")]
    p_type: MethodErrorType,
}

#[derive(Debug, Deserialize)]
pub enum MethodErrorType {
    #[serde(rename = "serverUnavailable")]
    ServerUnavailable,
    #[serde(rename = "serverFail")]
    ServerFail,
    #[serde(rename = "serverPartialFail")]
    ServerPartialFail,
    #[serde(rename = "unknownMethod")]
    UnknownMethod,
    #[serde(rename = "invalidArguments")]
    InvalidArguments,
    #[serde(rename = "invalidResultReference")]
    InvalidResultReference,
    #[serde(rename = "forbidden")]
    Forbidden,
    #[serde(rename = "accountNotFound")]
    AccountNotFound,
    #[serde(rename = "accountNotSupportedByMethod")]
    AccountNotSupportedByMethod,
    #[serde(rename = "accountReadOnly")]
    AccountReadOnly,
    #[serde(rename = "requestTooLarge")]
    RequestTooLarge,
    #[serde(rename = "cannotCalculateChanges")]
    CannotCalculateChanges,
    #[serde(rename = "stateMismatch")]
    StateMismatch,
    #[serde(rename = "alreadyExists")]
    AlreadyExists,
    #[serde(rename = "fromAccountNotFound")]
    FromAccountNotFound,
    #[serde(rename = "fromAccountNotSupportedByMethod")]
    FromAccountNotSupportedByMethod,
    #[serde(rename = "anchorNotFound")]
    AnchorNotFound,
    #[serde(rename = "unsupportedSort")]
    UnsupportedSort,
    #[serde(rename = "unsupportedFilter")]
    UnsupportedFilter,
    #[serde(rename = "tooManyChanges")]
    TooManyChanges,
}

impl ProblemDetails {
    pub fn error(&self) -> &ProblemType {
        &self.p_type
    }

    pub fn status(&self) -> Option<u32> {
        self.status
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }

    pub fn limit(&self) -> Option<usize> {
        self.limit
    }
}

impl MethodError {
    pub fn error(&self) -> &MethodErrorType {
        &self.p_type
    }
}

impl Display for ProblemDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.p_type {
            ProblemType::UnknownCapability => write!(f, "Unknown capability")?,
            ProblemType::NotJSON => write!(f, "Not JSON")?,
            ProblemType::NotRequest => write!(f, "Not request")?,
            ProblemType::Limit => write!(f, "Limit")?,
        }

        if let Some(status) = self.status {
            write!(f, " (status {})", status)?
        }

        if let Some(title) = &self.title {
            write!(f, ": {}", title)?
        }

        if let Some(detail) = &self.detail {
            write!(f, ". Details: {}", detail)?
        }

        Ok(())
    }
}
