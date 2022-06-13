use crate::{
    client::Client,
    core::{
        changes::{ChangesRequest, ChangesResponse},
        get::GetRequest,
        query::{QueryRequest, QueryResponse},
        query_changes::{QueryChangesRequest, QueryChangesResponse},
        request::{Arguments, Request},
        response::{PrincipalGetResponse, PrincipalSetResponse},
        set::SetRequest,
    },
    Get, Method, Set,
};

use super::Principal;

impl Client {
    pub async fn individual_create(
        &mut self,
        name: impl Into<String>,
        parent_id: Option<impl Into<String>>,
    ) -> crate::Result<Principal> {
        /*let mut request = self.build();
        let id = request
            .set_mailbox()
            .create()
            .name(name)
            .role(role)
            .parent_id(parent_id)
            .create_id()
            .unwrap();
        request
            .send_single::<MailboxSetResponse>()
            .await?
            .created(&id)*/
        todo!()
    }
}

impl Request<'_> {
    pub fn get_principal(&mut self) -> &mut GetRequest<Principal<Set>> {
        self.add_method_call(
            Method::GetPrincipal,
            Arguments::principal_get(self.params(Method::GetPrincipal)),
        )
        .principal_get_mut()
    }

    pub async fn send_get_principal(self) -> crate::Result<PrincipalGetResponse> {
        self.send_single().await
    }

    pub fn changes_principal(&mut self, since_state: impl Into<String>) -> &mut ChangesRequest {
        self.add_method_call(
            Method::ChangesPrincipal,
            Arguments::changes(self.params(Method::ChangesPrincipal), since_state.into()),
        )
        .changes_mut()
    }

    pub async fn send_changes_principal(self) -> crate::Result<ChangesResponse<Principal<Get>>> {
        self.send_single().await
    }

    pub fn query_principal(&mut self) -> &mut QueryRequest<Principal<Set>> {
        self.add_method_call(
            Method::QueryPrincipal,
            Arguments::principal_query(self.params(Method::QueryPrincipal)),
        )
        .principal_query_mut()
    }

    pub async fn send_query_principal(self) -> crate::Result<QueryResponse> {
        self.send_single().await
    }

    pub fn query_principal_changes(
        &mut self,
        since_query_state: impl Into<String>,
    ) -> &mut QueryChangesRequest<Principal<Set>> {
        self.add_method_call(
            Method::QueryChangesPrincipal,
            Arguments::principal_query_changes(
                self.params(Method::QueryChangesPrincipal),
                since_query_state.into(),
            ),
        )
        .principal_query_changes_mut()
    }

    pub async fn send_query_principal_changes(self) -> crate::Result<QueryChangesResponse> {
        self.send_single().await
    }

    pub fn set_principal(&mut self) -> &mut SetRequest<Principal<Set>> {
        self.add_method_call(
            Method::SetPrincipal,
            Arguments::principal_set(self.params(Method::SetPrincipal)),
        )
        .principal_set_mut()
    }

    pub async fn send_set_principal(self) -> crate::Result<PrincipalSetResponse> {
        self.send_single().await
    }
}
