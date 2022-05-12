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
pub struct Response<T> {
    #[serde(rename = "methodResponses")]
    method_responses: Vec<T>,

    #[serde(rename = "createdIds")]
    created_ids: Option<HashMap<String, String>>,

    #[serde(rename = "sessionState")]
    session_state: String,
}

impl<T> Response<T> {
    pub fn method_responses(&self) -> &[T] {
        self.method_responses.as_ref()
    }

    pub fn unwrap_method_responses(self) -> Vec<T> {
        self.method_responses
    }

    pub fn created_ids(&self) -> Option<impl Iterator<Item = (&String, &String)>> {
        self.created_ids.as_ref().map(|map| map.iter())
    }

    pub fn session_state(&self) -> &str {
        &self.session_state
    }
}

impl Response<MethodResponse> {
    pub fn method_response(&self, id: &str) -> Option<&MethodResponse> {
        self.method_responses
            .iter()
            .find(|response| response.call_id() == id)
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SingleMethodResponse<T> {
    Error((Error, MethodError, String)),
    Ok((String, T, String)),
}

pub type PushSubscriptionSetResponse =
    SetResponse<PushSubscription<Get>, push_subscription::Property>;
pub type PushSubscriptionGetResponse = GetResponse<PushSubscription<Get>>;
pub type MaiboxChangesResponse = ChangesResponse<mailbox::ChangesResponse>;
pub type MailboxSetResponse = SetResponse<Mailbox<Get>, mailbox::Property>;
pub type MailboxGetResponse = GetResponse<Mailbox<Get>>;
pub type ThreadGetResponse = GetResponse<Thread>;
pub type ThreadChangesResponse = ChangesResponse<()>;
pub type EmailGetResponse = GetResponse<Email<Get>>;
pub type EmailSetResponse = SetResponse<Email<Get>, email::Property>;
pub type EmailCopyResponse = CopyResponse<Email<Get>, email::Property>;
pub type EmailChangesResponse = ChangesResponse<()>;
pub type SearchSnippetGetResponse = GetResponse<String>;
pub type IdentitySetResponse = SetResponse<Identity<Get>, identity::Property>;
pub type IdentityGetResponse = GetResponse<Identity<Get>>;
pub type IdentityChangesResponse = ChangesResponse<()>;
pub type EmailSubmissionSetResponse = SetResponse<EmailSubmission<Get>, email_submission::Property>;
pub type EmailSubmissionGetResponse = GetResponse<EmailSubmission<Get>>;
pub type EmailSubmissionChangesResponse = ChangesResponse<()>;
pub type VacationResponseGetResponse = GetResponse<VacationResponse<Get>>;
pub type VacationResponseSetResponse =
    SetResponse<VacationResponse<Get>, vacation_response::Property>;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MethodResponse {
    CopyBlob((CopyBlob, CopyBlobResponse, String)),
    GetPushSubscription((GetPushSubscription, PushSubscriptionGetResponse, String)),
    SetPushSubscription((SetPushSubscription, PushSubscriptionSetResponse, String)),
    GetMailbox((GetMailbox, MailboxGetResponse, String)),
    ChangesMailbox((ChangesMailbox, MaiboxChangesResponse, String)),
    QueryMailbox((QueryMailbox, QueryResponse, String)),
    QueryChangesMailbox((QueryChangesMailbox, QueryChangesResponse, String)),
    SetMailbox((SetMailbox, MailboxSetResponse, String)),
    GetThread((GetThread, ThreadGetResponse, String)),
    ChangesThread((ChangesThread, ThreadChangesResponse, String)),
    GetEmail((GetEmail, EmailGetResponse, String)),
    ChangesEmail((ChangesEmail, EmailChangesResponse, String)),
    QueryEmail((QueryEmail, QueryResponse, String)),
    QueryChangesEmail((QueryChangesEmail, QueryChangesResponse, String)),
    SetEmail((SetEmail, EmailSetResponse, String)),
    CopyEmail((CopyEmail, EmailCopyResponse, String)),
    ImportEmail((ImportEmail, EmailImportResponse, String)),
    ParseEmail((ParseEmail, EmailParseResponse, String)),
    GetSearchSnippet((GetSearchSnippet, SearchSnippetGetResponse, String)),
    GetIdentity((GetIdentity, IdentityGetResponse, String)),
    ChangesIdentity((ChangesIdentity, IdentityChangesResponse, String)),
    SetIdentity((SetIdentity, IdentitySetResponse, String)),
    GetEmailSubmission((GetEmailSubmission, EmailSubmissionGetResponse, String)),
    ChangesEmailSubmission(
        (
            ChangesEmailSubmission,
            EmailSubmissionChangesResponse,
            String,
        ),
    ),
    QueryEmailSubmission((QueryEmailSubmission, QueryResponse, String)),
    QueryChangesEmailSubmission((QueryChangesEmailSubmission, QueryChangesResponse, String)),
    SetEmailSubmission((SetEmailSubmission, EmailSubmissionSetResponse, String)),
    GetVacationResponse((GetVacationResponse, VacationResponseGetResponse, String)),
    SetVacationResponse((SetVacationResponse, VacationResponseSetResponse, String)),
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

    pub fn unwrap_copy_blob(self) -> crate::Result<CopyBlobResponse> {
        match self {
            Self::CopyBlob((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_push_subscription(self) -> crate::Result<PushSubscriptionGetResponse> {
        match self {
            Self::GetPushSubscription((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_push_subscription(self) -> crate::Result<PushSubscriptionSetResponse> {
        match self {
            Self::SetPushSubscription((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_mailbox(self) -> crate::Result<MailboxGetResponse> {
        match self {
            Self::GetMailbox((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_mailbox(self) -> crate::Result<MaiboxChangesResponse> {
        match self {
            Self::ChangesMailbox((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_mailbox(self) -> crate::Result<QueryResponse> {
        match self {
            Self::QueryMailbox((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_changes_mailbox(self) -> crate::Result<QueryChangesResponse> {
        match self {
            Self::QueryChangesMailbox((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_mailbox(self) -> crate::Result<MailboxSetResponse> {
        match self {
            Self::SetMailbox((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_thread(self) -> crate::Result<ThreadGetResponse> {
        match self {
            Self::GetThread((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_thread(self) -> crate::Result<ThreadChangesResponse> {
        match self {
            Self::ChangesThread((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_email(self) -> crate::Result<EmailGetResponse> {
        match self {
            Self::GetEmail((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_email(self) -> crate::Result<EmailChangesResponse> {
        match self {
            Self::ChangesEmail((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_email(self) -> crate::Result<QueryResponse> {
        match self {
            Self::QueryEmail((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_changes_email(self) -> crate::Result<QueryChangesResponse> {
        match self {
            Self::QueryChangesEmail((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_email(self) -> crate::Result<EmailSetResponse> {
        match self {
            Self::SetEmail((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_copy_email(self) -> crate::Result<EmailCopyResponse> {
        match self {
            Self::CopyEmail((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_import_email(self) -> crate::Result<EmailImportResponse> {
        match self {
            Self::ImportEmail((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_parse_email(self) -> crate::Result<EmailParseResponse> {
        match self {
            Self::ParseEmail((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_search_snippet(self) -> crate::Result<SearchSnippetGetResponse> {
        match self {
            Self::GetSearchSnippet((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_identity(self) -> crate::Result<IdentityGetResponse> {
        match self {
            Self::GetIdentity((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_identity(self) -> crate::Result<IdentityChangesResponse> {
        match self {
            Self::ChangesIdentity((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_identity(self) -> crate::Result<IdentitySetResponse> {
        match self {
            Self::SetIdentity((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_email_submission(self) -> crate::Result<EmailSubmissionGetResponse> {
        match self {
            Self::GetEmailSubmission((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_changes_email_submission(self) -> crate::Result<EmailSubmissionChangesResponse> {
        match self {
            Self::ChangesEmailSubmission((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_email_submission(self) -> crate::Result<EmailSubmissionSetResponse> {
        match self {
            Self::SetEmailSubmission((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_email_submission(self) -> crate::Result<QueryResponse> {
        match self {
            Self::QueryEmailSubmission((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_query_changes_email_submission(self) -> crate::Result<QueryChangesResponse> {
        match self {
            Self::QueryChangesEmailSubmission((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_get_vacation_response(self) -> crate::Result<VacationResponseGetResponse> {
        match self {
            Self::GetVacationResponse((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_set_vacation_response(self) -> crate::Result<VacationResponseSetResponse> {
        match self {
            Self::SetVacationResponse((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
        }
    }

    pub fn unwrap_echo(self) -> crate::Result<serde_json::Value> {
        match self {
            Self::Echo((_, response, _)) => Ok(response),
            Self::Error((_, err, _)) => Err(err.into()),
            _ => Err("Response type mismatch".into()),
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
