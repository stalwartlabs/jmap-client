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
    request_id: Option<String>,
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
    pub fn new(
        p_type: ProblemType,
        status: Option<u32>,
        title: Option<String>,
        detail: Option<String>,
        limit: Option<usize>,
        request_id: Option<String>,
    ) -> Self {
        ProblemDetails {
            p_type,
            status,
            title,
            detail,
            limit,
            request_id,
        }
    }

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

    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }
}

impl MethodError {
    pub fn error(&self) -> &MethodErrorType {
        &self.p_type
    }
}

impl Display for MethodError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.p_type {
            MethodErrorType::ServerUnavailable => write!(f, "Server unavailable"),
            MethodErrorType::ServerFail => write!(f, "Server fail"),
            MethodErrorType::ServerPartialFail => write!(f, "Server partial fail"),
            MethodErrorType::UnknownMethod => write!(f, "Unknown method"),
            MethodErrorType::InvalidArguments => write!(f, "Invalid arguments"),
            MethodErrorType::InvalidResultReference => write!(f, "Invalid result reference"),
            MethodErrorType::Forbidden => write!(f, "Forbidden"),
            MethodErrorType::AccountNotFound => write!(f, "Account not found"),
            MethodErrorType::AccountNotSupportedByMethod => {
                write!(f, "Account not supported by method")
            }
            MethodErrorType::AccountReadOnly => write!(f, "Account read only"),
            MethodErrorType::RequestTooLarge => write!(f, "Request too large"),
            MethodErrorType::CannotCalculateChanges => write!(f, "Cannot calculate changes"),
            MethodErrorType::StateMismatch => write!(f, "State mismatch"),
            MethodErrorType::AlreadyExists => write!(f, "Already exists"),
            MethodErrorType::FromAccountNotFound => write!(f, "From account not found"),
            MethodErrorType::FromAccountNotSupportedByMethod => {
                write!(f, "From account not supported by method")
            }
            MethodErrorType::AnchorNotFound => write!(f, "Anchor not found"),
            MethodErrorType::UnsupportedSort => write!(f, "Unsupported sort"),
            MethodErrorType::UnsupportedFilter => write!(f, "Unsupported filter"),
            MethodErrorType::TooManyChanges => write!(f, "Too many changes"),
        }
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