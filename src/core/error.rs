use serde::{Deserialize, Serialize};

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
