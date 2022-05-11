use std::{collections::HashMap, time::Duration};

use serde::Serialize;

use crate::{
    blob::copy::CopyBlobRequest,
    client::Client,
    email::{self, import::EmailImportRequest, parse::EmailParseRequest, Email},
    email_submission::{self, EmailSubmission},
    identity::{self, Identity},
    mailbox::{self, Mailbox},
    push_subscription::{self, PushSubscription},
    thread,
    vacation_response::{self, VacationResponse},
    Method, Set, URI,
};

use super::{
    changes::ChangesRequest, copy::CopyRequest, get::GetRequest, query::QueryRequest,
    query_changes::QueryChangesRequest, response::Response, set::SetRequest,
};

#[derive(Serialize)]
pub struct Request<'x> {
    #[serde(skip)]
    client: &'x mut Client,

    using: Vec<URI>,

    #[serde(rename = "methodCalls")]
    method_calls: Vec<(Method, Arguments, String)>,

    #[serde(rename = "createdIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    created_ids: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResultReference {
    #[serde(rename = "resultOf")]
    result_of: String,
    name: Method,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Arguments {
    Changes(ChangesRequest),
    PushGet(GetRequest<push_subscription::Property, ()>),
    PushSet(SetRequest<PushSubscription<Set>, ()>),
    BlobCopy(CopyBlobRequest),
    MailboxGet(GetRequest<mailbox::Property, ()>),
    MailboxQuery(
        QueryRequest<mailbox::query::Filter, mailbox::query::Comparator, mailbox::QueryArguments>,
    ),
    MailboxQueryChanges(
        QueryChangesRequest<
            mailbox::query::Filter,
            mailbox::query::Comparator,
            mailbox::QueryArguments,
        >,
    ),
    MailboxSet(SetRequest<Mailbox<Set>, mailbox::SetArguments>),
    ThreadGet(GetRequest<thread::Property, ()>),
    EmailGet(GetRequest<email::Property, email::GetArguments>),
    EmailQuery(QueryRequest<email::query::Filter, email::query::Comparator, email::QueryArguments>),
    EmailQueryChanges(
        QueryChangesRequest<email::query::Filter, email::query::Comparator, email::QueryArguments>,
    ),
    EmailSet(SetRequest<Email<Set>, ()>),
    EmailCopy(CopyRequest<Email<Set>>),
    EmailImport(EmailImportRequest),
    EmailParse(EmailParseRequest),
    IdentityGet(GetRequest<identity::Property, ()>),
    IdentitySet(SetRequest<Identity<Set>, ()>),
    EmailSubmissionGet(GetRequest<email_submission::Property, ()>),
    EmailSubmissionQuery(
        QueryRequest<email_submission::query::Filter, email_submission::query::Comparator, ()>,
    ),
    EmailSubmissionQueryChanges(
        QueryChangesRequest<
            email_submission::query::Filter,
            email_submission::query::Comparator,
            (),
        >,
    ),
    EmailSubmissionSet(SetRequest<EmailSubmission<Set>, email_submission::SetArguments>),
    VacationResponseGet(GetRequest<vacation_response::Property, ()>),
    VacationResponseSet(SetRequest<VacationResponse<Set>, ()>),
}

impl Arguments {
    pub fn changes(account_id: String, since_state: String) -> Self {
        Arguments::Changes(ChangesRequest::new(account_id, since_state))
    }

    pub fn push_get(account_id: String) -> Self {
        Arguments::PushGet(GetRequest::new(account_id))
    }

    pub fn push_set(account_id: String) -> Self {
        Arguments::PushSet(SetRequest::new(account_id))
    }

    pub fn blob_copy(from_account_id: String, account_id: String) -> Self {
        Arguments::BlobCopy(CopyBlobRequest::new(from_account_id, account_id))
    }

    pub fn mailbox_get(account_id: String) -> Self {
        Arguments::MailboxGet(GetRequest::new(account_id))
    }

    pub fn mailbox_query(account_id: String) -> Self {
        Arguments::MailboxQuery(QueryRequest::new(account_id))
    }

    pub fn mailbox_query_changes(account_id: String, since_query_state: String) -> Self {
        Arguments::MailboxQueryChanges(QueryChangesRequest::new(account_id, since_query_state))
    }

    pub fn mailbox_set(account_id: String) -> Self {
        Arguments::MailboxSet(SetRequest::new(account_id))
    }

    pub fn thread_get(account_id: String) -> Self {
        Arguments::ThreadGet(GetRequest::new(account_id))
    }

    pub fn email_get(account_id: String) -> Self {
        Arguments::EmailGet(GetRequest::new(account_id))
    }

    pub fn email_query(account_id: String) -> Self {
        Arguments::EmailQuery(QueryRequest::new(account_id))
    }

    pub fn email_query_changes(account_id: String, since_query_state: String) -> Self {
        Arguments::EmailQueryChanges(QueryChangesRequest::new(account_id, since_query_state))
    }

    pub fn email_set(account_id: String) -> Self {
        Arguments::EmailSet(SetRequest::new(account_id))
    }

    pub fn email_copy(from_account_id: String, account_id: String) -> Self {
        Arguments::EmailCopy(CopyRequest::new(from_account_id, account_id))
    }

    pub fn email_import(account_id: String) -> Self {
        Arguments::EmailImport(EmailImportRequest::new(account_id))
    }

    pub fn email_parse(account_id: String) -> Self {
        Arguments::EmailParse(EmailParseRequest::new(account_id))
    }

    pub fn identity_get(account_id: String) -> Self {
        Arguments::IdentityGet(GetRequest::new(account_id))
    }

    pub fn identity_set(account_id: String) -> Self {
        Arguments::IdentitySet(SetRequest::new(account_id))
    }

    pub fn email_submission_get(account_id: String) -> Self {
        Arguments::EmailSubmissionGet(GetRequest::new(account_id))
    }

    pub fn email_submission_query(account_id: String) -> Self {
        Arguments::EmailSubmissionQuery(QueryRequest::new(account_id))
    }

    pub fn email_submission_query_changes(account_id: String, since_query_state: String) -> Self {
        Arguments::EmailSubmissionQueryChanges(QueryChangesRequest::new(
            account_id,
            since_query_state,
        ))
    }

    pub fn email_submission_set(account_id: String) -> Self {
        Arguments::EmailSubmissionSet(SetRequest::new(account_id))
    }

    pub fn vacation_response_get(account_id: String) -> Self {
        Arguments::VacationResponseGet(GetRequest::new(account_id))
    }

    pub fn vacation_response_set(account_id: String) -> Self {
        Arguments::VacationResponseSet(SetRequest::new(account_id))
    }

    pub fn changes_mut(&mut self) -> &mut ChangesRequest {
        match self {
            Arguments::Changes(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn push_get_mut(&mut self) -> &mut GetRequest<push_subscription::Property, ()> {
        match self {
            Arguments::PushGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn push_set_mut(&mut self) -> &mut SetRequest<PushSubscription<Set>, ()> {
        match self {
            Arguments::PushSet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn blob_copy_mut(&mut self) -> &mut CopyBlobRequest {
        match self {
            Arguments::BlobCopy(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn mailbox_get_mut(&mut self) -> &mut GetRequest<mailbox::Property, ()> {
        match self {
            Arguments::MailboxGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn mailbox_query_mut(
        &mut self,
    ) -> &mut QueryRequest<
        mailbox::query::Filter,
        mailbox::query::Comparator,
        mailbox::QueryArguments,
    > {
        match self {
            Arguments::MailboxQuery(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn mailbox_query_changes_mut(
        &mut self,
    ) -> &mut QueryChangesRequest<
        mailbox::query::Filter,
        mailbox::query::Comparator,
        mailbox::QueryArguments,
    > {
        match self {
            Arguments::MailboxQueryChanges(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn mailbox_set_mut(&mut self) -> &mut SetRequest<Mailbox<Set>, mailbox::SetArguments> {
        match self {
            Arguments::MailboxSet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn thread_get_mut(&mut self) -> &mut GetRequest<thread::Property, ()> {
        match self {
            Arguments::ThreadGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn email_get_mut(&mut self) -> &mut GetRequest<email::Property, email::GetArguments> {
        match self {
            Arguments::EmailGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn email_query_mut(
        &mut self,
    ) -> &mut QueryRequest<email::query::Filter, email::query::Comparator, email::QueryArguments>
    {
        match self {
            Arguments::EmailQuery(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn email_query_changes_mut(
        &mut self,
    ) -> &mut QueryChangesRequest<
        email::query::Filter,
        email::query::Comparator,
        email::QueryArguments,
    > {
        match self {
            Arguments::EmailQueryChanges(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn email_set_mut(&mut self) -> &mut SetRequest<Email<Set>, ()> {
        match self {
            Arguments::EmailSet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn email_copy_mut(&mut self) -> &mut CopyRequest<Email<Set>> {
        match self {
            Arguments::EmailCopy(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn email_import_mut(&mut self) -> &mut EmailImportRequest {
        match self {
            Arguments::EmailImport(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn email_parse_mut(&mut self) -> &mut EmailParseRequest {
        match self {
            Arguments::EmailParse(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn identity_get_mut(&mut self) -> &mut GetRequest<identity::Property, ()> {
        match self {
            Arguments::IdentityGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn identity_set_mut(&mut self) -> &mut SetRequest<Identity<Set>, ()> {
        match self {
            Arguments::IdentitySet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn email_submission_get_mut(&mut self) -> &mut GetRequest<email_submission::Property, ()> {
        match self {
            Arguments::EmailSubmissionGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn email_submission_query_mut(
        &mut self,
    ) -> &mut QueryRequest<email_submission::query::Filter, email_submission::query::Comparator, ()>
    {
        match self {
            Arguments::EmailSubmissionQuery(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn email_submission_query_changes_mut(
        &mut self,
    ) -> &mut QueryChangesRequest<
        email_submission::query::Filter,
        email_submission::query::Comparator,
        (),
    > {
        match self {
            Arguments::EmailSubmissionQueryChanges(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn email_submission_set_mut(
        &mut self,
    ) -> &mut SetRequest<EmailSubmission<Set>, email_submission::SetArguments> {
        match self {
            Arguments::EmailSubmissionSet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn vacation_response_get_mut(
        &mut self,
    ) -> &mut GetRequest<vacation_response::Property, ()> {
        match self {
            Arguments::VacationResponseGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn vacation_response_set_mut(&mut self) -> &mut SetRequest<VacationResponse<Set>, ()> {
        match self {
            Arguments::VacationResponseSet(ref mut r) => r,
            _ => unreachable!(),
        }
    }
}

impl<'x> Request<'x> {
    pub fn new(client: &'x mut Client) -> Self {
        Request {
            using: vec![URI::Core, URI::Mail],
            method_calls: vec![],
            created_ids: None,
            client,
        }
    }

    pub async fn send(&mut self) -> crate::Result<Response> {
        let response: Response = serde_json::from_slice(
            &Client::handle_error(
                reqwest::Client::builder()
                    .timeout(Duration::from_millis(self.client.timeout()))
                    .default_headers(self.client.headers().clone())
                    .build()?
                    .post(self.client.session().api_url())
                    .body(serde_json::to_string(&self)?)
                    .send()
                    .await?,
            )
            .await?
            .bytes()
            .await?,
        )?;
        self.client.update_session(response.session_state()).await?;
        Ok(response)
    }

    fn add_method_call(&mut self, method: Method, arguments: Arguments) -> &mut Arguments {
        let call_id = format!("s{}", self.method_calls.len());
        self.method_calls.push((method, arguments, call_id));
        &mut self.method_calls.last_mut().unwrap().1
    }

    pub fn get_push(&mut self) -> &mut GetRequest<push_subscription::Property, ()> {
        self.add_method_call(
            Method::GetPushSubscription,
            Arguments::push_get(self.client.default_account_id().to_string()),
        )
        .push_get_mut()
    }

    pub fn set_push(&mut self) -> &mut SetRequest<PushSubscription<Set>, ()> {
        self.add_method_call(
            Method::SetPushSubscription,
            Arguments::push_set(self.client.default_account_id().to_string()),
        )
        .push_set_mut()
    }

    pub fn copy_blob(
        &mut self,
        from_account_id: impl Into<String>,
        account_id: impl Into<String>,
    ) -> &mut CopyBlobRequest {
        self.add_method_call(
            Method::CopyBlob,
            Arguments::blob_copy(from_account_id.into(), account_id.into()),
        )
        .blob_copy_mut()
    }

    pub fn get_mailbox(&mut self) -> &mut GetRequest<mailbox::Property, ()> {
        self.add_method_call(
            Method::GetMailbox,
            Arguments::mailbox_get(self.client.default_account_id().to_string()),
        )
        .mailbox_get_mut()
    }

    pub fn changes_mailbox(&mut self, since_state: impl Into<String>) -> &mut ChangesRequest {
        self.add_method_call(
            Method::ChangesMailbox,
            Arguments::changes(
                self.client.default_account_id().to_string(),
                since_state.into(),
            ),
        )
        .changes_mut()
    }

    pub fn query_mailbox(
        &mut self,
    ) -> &mut QueryRequest<
        mailbox::query::Filter,
        mailbox::query::Comparator,
        mailbox::QueryArguments,
    > {
        self.add_method_call(
            Method::QueryMailbox,
            Arguments::mailbox_query(self.client.default_account_id().to_string()),
        )
        .mailbox_query_mut()
    }

    pub fn query_mailbox_changes(
        &mut self,
        since_query_state: impl Into<String>,
    ) -> &mut QueryChangesRequest<
        mailbox::query::Filter,
        mailbox::query::Comparator,
        mailbox::QueryArguments,
    > {
        self.add_method_call(
            Method::QueryChangesMailbox,
            Arguments::mailbox_query_changes(
                self.client.default_account_id().to_string(),
                since_query_state.into(),
            ),
        )
        .mailbox_query_changes_mut()
    }

    pub fn set_mailbox(&mut self) -> &mut SetRequest<Mailbox<Set>, mailbox::SetArguments> {
        self.add_method_call(
            Method::SetMailbox,
            Arguments::mailbox_set(self.client.default_account_id().to_string()),
        )
        .mailbox_set_mut()
    }

    pub fn get_thread(&mut self) -> &mut GetRequest<thread::Property, ()> {
        self.add_method_call(
            Method::GetThread,
            Arguments::thread_get(self.client.default_account_id().to_string()),
        )
        .thread_get_mut()
    }

    pub fn changes_thread(&mut self, since_state: impl Into<String>) -> &mut ChangesRequest {
        self.add_method_call(
            Method::ChangesThread,
            Arguments::changes(
                self.client.default_account_id().to_string(),
                since_state.into(),
            ),
        )
        .changes_mut()
    }
    pub fn get_email(&mut self) -> &mut GetRequest<email::Property, email::GetArguments> {
        self.add_method_call(
            Method::GetEmail,
            Arguments::email_get(self.client.default_account_id().to_string()),
        )
        .email_get_mut()
    }

    pub fn changes_email(&mut self, since_state: impl Into<String>) -> &mut ChangesRequest {
        self.add_method_call(
            Method::ChangesEmail,
            Arguments::changes(
                self.client.default_account_id().to_string(),
                since_state.into(),
            ),
        )
        .changes_mut()
    }

    pub fn query_email(
        &mut self,
    ) -> &mut QueryRequest<email::query::Filter, email::query::Comparator, email::QueryArguments>
    {
        self.add_method_call(
            Method::QueryEmail,
            Arguments::email_query(self.client.default_account_id().to_string()),
        )
        .email_query_mut()
    }

    pub fn query_email_changes(
        &mut self,
        since_query_state: impl Into<String>,
    ) -> &mut QueryChangesRequest<
        email::query::Filter,
        email::query::Comparator,
        email::QueryArguments,
    > {
        self.add_method_call(
            Method::QueryChangesEmail,
            Arguments::email_query_changes(
                self.client.default_account_id().to_string(),
                since_query_state.into(),
            ),
        )
        .email_query_changes_mut()
    }

    pub fn set_email(&mut self) -> &mut SetRequest<Email<Set>, ()> {
        self.add_method_call(
            Method::SetEmail,
            Arguments::email_set(self.client.default_account_id().to_string()),
        )
        .email_set_mut()
    }

    pub fn copy_email(
        &mut self,
        from_account_id: impl Into<String>,
        account_id: impl Into<String>,
    ) -> &mut CopyRequest<Email<Set>> {
        self.add_method_call(
            Method::CopyEmail,
            Arguments::email_copy(from_account_id.into(), account_id.into()),
        )
        .email_copy_mut()
    }

    pub fn import_email(&mut self) -> &mut EmailImportRequest {
        self.add_method_call(
            Method::ImportEmail,
            Arguments::email_import(self.client.default_account_id().to_string()),
        )
        .email_import_mut()
    }

    pub fn parse_email(&mut self) -> &mut EmailParseRequest {
        self.add_method_call(
            Method::ParseEmail,
            Arguments::email_parse(self.client.default_account_id().to_string()),
        )
        .email_parse_mut()
    }

    pub fn get_identity(&mut self) -> &mut GetRequest<identity::Property, ()> {
        self.add_method_call(
            Method::GetIdentity,
            Arguments::identity_get(self.client.default_account_id().to_string()),
        )
        .identity_get_mut()
    }

    pub fn set_identity(&mut self) -> &mut SetRequest<Identity<Set>, ()> {
        self.add_method_call(
            Method::SetIdentity,
            Arguments::identity_set(self.client.default_account_id().to_string()),
        )
        .identity_set_mut()
    }

    pub fn get_email_submission(&mut self) -> &mut GetRequest<email_submission::Property, ()> {
        if !self.using.contains(&URI::Submission) {
            self.using.push(URI::Submission);
        }
        self.add_method_call(
            Method::GetEmailSubmission,
            Arguments::email_submission_get(self.client.default_account_id().to_string()),
        )
        .email_submission_get_mut()
    }

    pub fn changes_email_submission(
        &mut self,
        since_state: impl Into<String>,
    ) -> &mut ChangesRequest {
        if !self.using.contains(&URI::Submission) {
            self.using.push(URI::Submission);
        }
        self.add_method_call(
            Method::ChangesEmailSubmission,
            Arguments::changes(
                self.client.default_account_id().to_string(),
                since_state.into(),
            ),
        )
        .changes_mut()
    }

    pub fn query_email_submission(
        &mut self,
    ) -> &mut QueryRequest<email_submission::query::Filter, email_submission::query::Comparator, ()>
    {
        if !self.using.contains(&URI::Submission) {
            self.using.push(URI::Submission);
        }
        self.add_method_call(
            Method::QueryEmailSubmission,
            Arguments::email_submission_query(self.client.default_account_id().to_string()),
        )
        .email_submission_query_mut()
    }

    pub fn query_email_submission_changes(
        &mut self,
        since_query_state: impl Into<String>,
    ) -> &mut QueryChangesRequest<
        email_submission::query::Filter,
        email_submission::query::Comparator,
        (),
    > {
        if !self.using.contains(&URI::Submission) {
            self.using.push(URI::Submission);
        }
        self.add_method_call(
            Method::QueryChangesEmailSubmission,
            Arguments::email_submission_query_changes(
                self.client.default_account_id().to_string(),
                since_query_state.into(),
            ),
        )
        .email_submission_query_changes_mut()
    }

    pub fn set_email_submission(
        &mut self,
    ) -> &mut SetRequest<EmailSubmission<Set>, email_submission::SetArguments> {
        if !self.using.contains(&URI::Submission) {
            self.using.push(URI::Submission);
        }
        self.add_method_call(
            Method::SetEmailSubmission,
            Arguments::email_submission_set(self.client.default_account_id().to_string()),
        )
        .email_submission_set_mut()
    }

    pub fn get_vacation_response(&mut self) -> &mut GetRequest<vacation_response::Property, ()> {
        if !self.using.contains(&URI::VacationResponse) {
            self.using.push(URI::VacationResponse);
        }
        self.add_method_call(
            Method::GetVacationResponse,
            Arguments::vacation_response_get(self.client.default_account_id().to_string()),
        )
        .vacation_response_get_mut()
    }

    pub fn set_vacation_response(&mut self) -> &mut SetRequest<VacationResponse<Set>, ()> {
        if !self.using.contains(&URI::VacationResponse) {
            self.using.push(URI::VacationResponse);
        }
        self.add_method_call(
            Method::SetVacationResponse,
            Arguments::vacation_response_set(self.client.default_account_id().to_string()),
        )
        .vacation_response_set_mut()
    }

    pub fn result_reference(&self, path: impl Into<String>) -> ResultReference {
        let last_method = self.method_calls.last().unwrap();
        ResultReference {
            result_of: last_method.2.clone(),
            name: last_method.0.clone(),
            path: path.into(),
        }
    }
}
