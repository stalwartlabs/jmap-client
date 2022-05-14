use crate::{
    core::{
        changes::{ChangesRequest, ChangesResponse},
        get::GetRequest,
        query::{QueryRequest, QueryResponse},
        query_changes::{QueryChangesRequest, QueryChangesResponse},
        request::{Arguments, Request},
        response::{EmailSubmissionGetResponse, EmailSubmissionSetResponse},
        set::SetRequest,
    },
    Method, Set, URI,
};

use super::EmailSubmission;

impl Request<'_> {
    pub fn get_email_submission(&mut self) -> &mut GetRequest<super::Property, ()> {
        self.add_capability(URI::Submission);
        self.add_method_call(
            Method::GetEmailSubmission,
            Arguments::email_submission_get(self.params(Method::GetEmailSubmission)),
        )
        .email_submission_get_mut()
    }

    pub async fn send_get_email_submission(self) -> crate::Result<EmailSubmissionGetResponse> {
        self.send_single().await
    }

    pub fn changes_email_submission(
        &mut self,
        since_state: impl Into<String>,
    ) -> &mut ChangesRequest {
        self.add_capability(URI::Submission);
        self.add_method_call(
            Method::ChangesEmailSubmission,
            Arguments::changes(
                self.params(Method::ChangesEmailSubmission),
                since_state.into(),
            ),
        )
        .changes_mut()
    }

    pub async fn send_changes_email_submission(self) -> crate::Result<ChangesResponse<()>> {
        self.send_single().await
    }

    pub fn query_email_submission(
        &mut self,
    ) -> &mut QueryRequest<super::query::Filter, super::query::Comparator, ()> {
        self.add_capability(URI::Submission);
        self.add_method_call(
            Method::QueryEmailSubmission,
            Arguments::email_submission_query(self.params(Method::QueryEmailSubmission)),
        )
        .email_submission_query_mut()
    }

    pub async fn send_query_email_submission(self) -> crate::Result<QueryResponse> {
        self.send_single().await
    }

    pub fn query_email_submission_changes(
        &mut self,
        since_query_state: impl Into<String>,
    ) -> &mut QueryChangesRequest<super::query::Filter, super::query::Comparator, ()> {
        self.add_capability(URI::Submission);
        self.add_method_call(
            Method::QueryChangesEmailSubmission,
            Arguments::email_submission_query_changes(
                self.params(Method::QueryChangesEmailSubmission),
                since_query_state.into(),
            ),
        )
        .email_submission_query_changes_mut()
    }

    pub async fn send_query_email_submission_changes(self) -> crate::Result<QueryChangesResponse> {
        self.send_single().await
    }

    pub fn set_email_submission(
        &mut self,
    ) -> &mut SetRequest<EmailSubmission<Set>, super::SetArguments> {
        self.add_capability(URI::Submission);
        self.add_method_call(
            Method::SetEmailSubmission,
            Arguments::email_submission_set(self.params(Method::SetEmailSubmission)),
        )
        .email_submission_set_mut()
    }

    pub async fn send_set_email_submission(self) -> crate::Result<EmailSubmissionSetResponse> {
        self.send_single().await
    }
}
