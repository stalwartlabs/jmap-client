use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    blob::copy::CopyBlobResponse,
    email::{self, import::EmailImportResponse, parse::EmailParseResponse, Email},
    email_submission::{self, EmailSubmission},
    identity::{self, Identity},
    mailbox::{self, Mailbox},
    push_subscription::{self, PushSubscription},
    thread::Thread,
    vacation_response::{self, VacationResponse},
    Get, Method,
};

use super::{
    changes::ChangesResponse, copy::CopyResponse, error::MethodError, get::GetResponse,
    query::QueryResponse, query_changes::QueryChangesResponse, set::SetResponse,
};

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(rename = "methodResponses")]
    method_responses: Vec<MethodResponse>,

    #[serde(rename = "createdIds")]
    created_ids: Option<HashMap<String, String>>,

    #[serde(rename = "sessionState")]
    session_state: String,
}

impl Response {
    pub fn method_responses(&self) -> &[MethodResponse] {
        self.method_responses.as_ref()
    }

    pub fn method_response(&self, id: &str) -> Option<&MethodResponse> {
        self.method_responses
            .iter()
            .find(|response| response.call_id() == id)
    }

    pub fn created_ids(&self) -> Option<impl Iterator<Item = (&String, &String)>> {
        self.created_ids.as_ref().map(|map| map.iter())
    }

    pub fn session_state(&self) -> &str {
        &self.session_state
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MethodResponse {
    CopyBlob((CopyBlob, CopyBlobResponse, String)),
    GetPushSubscription(
        (
            GetPushSubscription,
            GetResponse<PushSubscription<Get>>,
            String,
        ),
    ),
    SetPushSubscription(
        (
            SetPushSubscription,
            SetResponse<PushSubscription<Get>, push_subscription::Property>,
            String,
        ),
    ),
    GetMailbox((GetMailbox, GetResponse<Mailbox<Get>>, String)),
    ChangesMailbox(
        (
            ChangesMailbox,
            ChangesResponse<mailbox::ChangesResponse>,
            String,
        ),
    ),
    QueryMailbox((QueryMailbox, QueryResponse, String)),
    QueryChangesMailbox((QueryChangesMailbox, QueryChangesResponse, String)),
    SetMailbox(
        (
            SetMailbox,
            SetResponse<Mailbox<Get>, mailbox::Property>,
            String,
        ),
    ),
    GetThread((GetThread, GetResponse<Thread>, String)),
    ChangesThread((ChangesThread, ChangesResponse<()>, String)),
    GetEmail((GetEmail, GetResponse<Email<Get>>, String)),
    ChangesEmail((ChangesEmail, ChangesResponse<()>, String)),
    QueryEmail((QueryEmail, QueryResponse, String)),
    QueryChangesEmail((QueryChangesEmail, QueryChangesResponse, String)),
    SetEmail((SetEmail, SetResponse<Email<Get>, email::Property>, String)),
    CopyEmail((CopyEmail, CopyResponse<Email<Get>, email::Property>, String)),
    ImportEmail((ImportEmail, EmailImportResponse, String)),
    ParseEmail((ParseEmail, EmailParseResponse, String)),
    GetSearchSnippet((GetSearchSnippet, GetResponse<String>, String)),
    GetIdentity((GetIdentity, GetResponse<Identity<Get>>, String)),
    ChangesIdentity((ChangesIdentity, ChangesResponse<()>, String)),
    SetIdentity(
        (
            SetIdentity,
            SetResponse<Identity<Get>, identity::Property>,
            String,
        ),
    ),
    GetEmailSubmission(
        (
            GetEmailSubmission,
            GetResponse<EmailSubmission<Get>>,
            String,
        ),
    ),
    ChangesEmailSubmission((ChangesEmailSubmission, ChangesResponse<()>, String)),
    QueryEmailSubmission((QueryEmailSubmission, QueryResponse, String)),
    QueryChangesEmailSubmission((QueryChangesEmailSubmission, QueryChangesResponse, String)),
    SetEmailSubmission(
        (
            SetEmailSubmission,
            SetResponse<EmailSubmission<Get>, email_submission::Property>,
            String,
        ),
    ),
    GetVacationResponse(
        (
            GetVacationResponse,
            GetResponse<VacationResponse<Get>>,
            String,
        ),
    ),
    SetVacationResponse(
        (
            SetVacationResponse,
            SetResponse<VacationResponse<Get>, vacation_response::Property>,
            String,
        ),
    ),
    Echo((Echo, serde_json::Value, String)),
    Error((Error, MethodError, String)),
}

impl MethodResponse {
    pub fn call_id(&self) -> &str {
        match self {
            Self::CopyBlob((_, _, id)) => id,
            Self::GetPushSubscription((_, _, id)) => id,
            Self::SetPushSubscription((_, _, id)) => id,
            Self::GetMailbox((_, _, id)) => id,
            Self::ChangesMailbox((_, _, id)) => id,
            Self::QueryMailbox((_, _, id)) => id,
            Self::QueryChangesMailbox((_, _, id)) => id,
            Self::SetMailbox((_, _, id)) => id,
            Self::GetThread((_, _, id)) => id,
            Self::ChangesThread((_, _, id)) => id,
            Self::GetEmail((_, _, id)) => id,
            Self::ChangesEmail((_, _, id)) => id,
            Self::QueryEmail((_, _, id)) => id,
            Self::QueryChangesEmail((_, _, id)) => id,
            Self::SetEmail((_, _, id)) => id,
            Self::CopyEmail((_, _, id)) => id,
            Self::ImportEmail((_, _, id)) => id,
            Self::ParseEmail((_, _, id)) => id,
            Self::GetSearchSnippet((_, _, id)) => id,
            Self::GetIdentity((_, _, id)) => id,
            Self::ChangesIdentity((_, _, id)) => id,
            Self::SetIdentity((_, _, id)) => id,
            Self::GetEmailSubmission((_, _, id)) => id,
            Self::ChangesEmailSubmission((_, _, id)) => id,
            Self::QueryEmailSubmission((_, _, id)) => id,
            Self::QueryChangesEmailSubmission((_, _, id)) => id,
            Self::SetEmailSubmission((_, _, id)) => id,
            Self::GetVacationResponse((_, _, id)) => id,
            Self::SetVacationResponse((_, _, id)) => id,
            Self::Echo((_, _, id)) => id,
            Self::Error((_, _, id)) => id,
        }
    }

    pub fn is_type(&self, type_: Method) -> bool {
        matches!(
            (self, type_),
            (Self::CopyBlob(_), Method::CopyBlob)
                | (Self::GetPushSubscription(_), Method::GetPushSubscription)
                | (Self::SetPushSubscription(_), Method::SetPushSubscription)
                | (Self::GetMailbox(_), Method::GetMailbox)
                | (Self::ChangesMailbox(_), Method::ChangesMailbox)
                | (Self::QueryMailbox(_), Method::QueryMailbox)
                | (Self::QueryChangesMailbox(_), Method::QueryChangesMailbox)
                | (Self::SetMailbox(_), Method::SetMailbox)
                | (Self::GetThread(_), Method::GetThread)
                | (Self::ChangesThread(_), Method::ChangesThread)
                | (Self::GetEmail(_), Method::GetEmail)
                | (Self::ChangesEmail(_), Method::ChangesEmail)
                | (Self::QueryEmail(_), Method::QueryEmail)
                | (Self::QueryChangesEmail(_), Method::QueryChangesEmail)
                | (Self::SetEmail(_), Method::SetEmail)
                | (Self::CopyEmail(_), Method::CopyEmail)
                | (Self::ImportEmail(_), Method::ImportEmail)
                | (Self::ParseEmail(_), Method::ParseEmail)
                | (Self::GetSearchSnippet(_), Method::GetSearchSnippet)
                | (Self::GetIdentity(_), Method::GetIdentity)
                | (Self::ChangesIdentity(_), Method::ChangesIdentity)
                | (Self::SetIdentity(_), Method::SetIdentity)
                | (Self::GetEmailSubmission(_), Method::GetEmailSubmission)
                | (
                    Self::ChangesEmailSubmission(_),
                    Method::ChangesEmailSubmission
                )
                | (Self::QueryEmailSubmission(_), Method::QueryEmailSubmission)
                | (
                    Self::QueryChangesEmailSubmission(_),
                    Method::QueryChangesEmailSubmission
                )
                | (Self::SetEmailSubmission(_), Method::SetEmailSubmission)
                | (Self::GetVacationResponse(_), Method::GetVacationResponse)
                | (Self::SetVacationResponse(_), Method::SetVacationResponse)
                | (Self::Echo(_), Method::Echo)
                | (Self::Error(_), Method::Error)
        )
    }

    pub fn as_copy_copy(&self) -> Option<&CopyBlobResponse> {
        match self {
            Self::CopyBlob((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_get_push_subscription(&self) -> Option<&GetResponse<PushSubscription<Get>>> {
        match self {
            Self::GetPushSubscription((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_set_push_subscription(
        &self,
    ) -> Option<&SetResponse<PushSubscription<Get>, push_subscription::Property>> {
        match self {
            Self::SetPushSubscription((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_get_mailbox(&self) -> Option<&GetResponse<Mailbox<Get>>> {
        match self {
            Self::GetMailbox((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_changes_mailbox(&self) -> Option<&ChangesResponse<mailbox::ChangesResponse>> {
        match self {
            Self::ChangesMailbox((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_query_mailbox(&self) -> Option<&QueryResponse> {
        match self {
            Self::QueryMailbox((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_query_changes_mailbox(&self) -> Option<&QueryChangesResponse> {
        match self {
            Self::QueryChangesMailbox((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_set_mailbox(&self) -> Option<&SetResponse<Mailbox<Get>, mailbox::Property>> {
        match self {
            Self::SetMailbox((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_get_thread(&self) -> Option<&GetResponse<Thread>> {
        match self {
            Self::GetThread((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_changes_thread(&self) -> Option<&ChangesResponse<()>> {
        match self {
            Self::ChangesThread((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_get_email(&self) -> Option<&GetResponse<Email>> {
        match self {
            Self::GetEmail((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_changes_email(&self) -> Option<&ChangesResponse<()>> {
        match self {
            Self::ChangesEmail((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_query_email(&self) -> Option<&QueryResponse> {
        match self {
            Self::QueryEmail((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_query_changes_email(&self) -> Option<&QueryChangesResponse> {
        match self {
            Self::QueryChangesEmail((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_set_email(&self) -> Option<&SetResponse<Email, email::Property>> {
        match self {
            Self::SetEmail((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_copy_email(&self) -> Option<&CopyResponse<Email<Get>, email::Property>> {
        match self {
            Self::CopyEmail((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_import_email(&self) -> Option<&EmailImportResponse> {
        match self {
            Self::ImportEmail((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_parse_email(&self) -> Option<&EmailParseResponse> {
        match self {
            Self::ParseEmail((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_get_search_snippet(&self) -> Option<&GetResponse<String>> {
        match self {
            Self::GetSearchSnippet((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_get_identity(&self) -> Option<&GetResponse<Identity>> {
        match self {
            Self::GetIdentity((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_changes_identity(&self) -> Option<&ChangesResponse<()>> {
        match self {
            Self::ChangesIdentity((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_set_identity(&self) -> Option<&SetResponse<Identity, identity::Property>> {
        match self {
            Self::SetIdentity((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_get_email_submission(&self) -> Option<&GetResponse<EmailSubmission>> {
        match self {
            Self::GetEmailSubmission((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_changes_email_submission(&self) -> Option<&ChangesResponse<()>> {
        match self {
            Self::ChangesEmailSubmission((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_set_email_submission(
        &self,
    ) -> Option<&SetResponse<EmailSubmission, email_submission::Property>> {
        match self {
            Self::SetEmailSubmission((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_query_email_submission(&self) -> Option<&QueryResponse> {
        match self {
            Self::QueryEmailSubmission((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_query_changes_email_submission(&self) -> Option<&QueryChangesResponse> {
        match self {
            Self::QueryChangesEmailSubmission((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_get_vacation_response(&self) -> Option<&GetResponse<VacationResponse>> {
        match self {
            Self::GetVacationResponse((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_set_vacation_response(
        &self,
    ) -> Option<&SetResponse<VacationResponse, vacation_response::Property>> {
        match self {
            Self::SetVacationResponse((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_echo(&self) -> Option<&serde_json::Value> {
        match self {
            Self::Echo((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn as_error(&self) -> Option<&MethodError> {
        match self {
            Self::Error((_, response, _)) => response.into(),
            _ => None,
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }
}

#[derive(Debug, Deserialize)]
pub enum Echo {
    #[serde(rename = "Core/echo")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum CopyBlob {
    #[serde(rename = "Blob/copy")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum GetPushSubscription {
    #[serde(rename = "PushSubscription/get")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum SetPushSubscription {
    #[serde(rename = "PushSubscription/set")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum GetMailbox {
    #[serde(rename = "Mailbox/get")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum ChangesMailbox {
    #[serde(rename = "Mailbox/changes")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum QueryMailbox {
    #[serde(rename = "Mailbox/query")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum QueryChangesMailbox {
    #[serde(rename = "Mailbox/queryChanges")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum SetMailbox {
    #[serde(rename = "Mailbox/set")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum GetThread {
    #[serde(rename = "Thread/get")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum ChangesThread {
    #[serde(rename = "Thread/changes")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum GetEmail {
    #[serde(rename = "Email/get")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum ChangesEmail {
    #[serde(rename = "Email/changes")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum QueryEmail {
    #[serde(rename = "Email/query")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum QueryChangesEmail {
    #[serde(rename = "Email/queryChanges")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum SetEmail {
    #[serde(rename = "Email/set")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum CopyEmail {
    #[serde(rename = "Email/copy")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum ImportEmail {
    #[serde(rename = "Email/import")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum ParseEmail {
    #[serde(rename = "Email/parse")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum GetSearchSnippet {
    #[serde(rename = "SearchSnippet/get")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum GetIdentity {
    #[serde(rename = "Identity/get")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum ChangesIdentity {
    #[serde(rename = "Identity/changes")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum SetIdentity {
    #[serde(rename = "Identity/set")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum GetEmailSubmission {
    #[serde(rename = "EmailSubmission/get")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum ChangesEmailSubmission {
    #[serde(rename = "EmailSubmission/changes")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum QueryEmailSubmission {
    #[serde(rename = "EmailSubmission/query")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum QueryChangesEmailSubmission {
    #[serde(rename = "EmailSubmission/queryChanges")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum SetEmailSubmission {
    #[serde(rename = "EmailSubmission/set")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum GetVacationResponse {
    #[serde(rename = "VacationResponse/get")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum SetVacationResponse {
    #[serde(rename = "VacationResponse/set")]
    V,
}

#[derive(Debug, Deserialize)]
pub enum Error {
    #[serde(rename = "error")]
    V,
}
