use crate::{
    client::Client,
    core::{
        changes::{ChangesRequest, ChangesResponse},
        get::GetRequest,
        query::{Comparator, Filter, QueryRequest, QueryResponse},
        query_changes::{QueryChangesRequest, QueryChangesResponse},
        request::{Arguments, Request},
        response::{PrincipalGetResponse, PrincipalSetResponse},
        set::{SetObject, SetRequest},
    },
    Get, Method, Set,
};

use super::{Principal, Property, Type};

impl Client {
    pub async fn individual_create(
        &mut self,
        email: impl Into<String>,
        secret: impl Into<String>,
        name: impl Into<String>,
    ) -> crate::Result<Principal> {
        let mut request = self.build();
        let id = request
            .set_principal()
            .create()
            .name(name)
            .secret(secret)
            .email(email)
            .ptype(Type::Individual)
            .create_id()
            .unwrap();
        request
            .send_single::<PrincipalSetResponse>()
            .await?
            .created(&id)
    }

    pub async fn domain_create(&mut self, name: impl Into<String>) -> crate::Result<Principal> {
        let mut request = self.build();
        let id = request
            .set_principal()
            .create()
            .name(name)
            .ptype(Type::Domain)
            .create_id()
            .unwrap();
        request
            .send_single::<PrincipalSetResponse>()
            .await?
            .created(&id)
    }

    pub async fn list_create(
        &mut self,
        email: impl Into<String>,
        name: impl Into<String>,
        members: impl IntoIterator<Item = impl Into<String>>,
    ) -> crate::Result<Principal> {
        let mut request = self.build();
        let id = request
            .set_principal()
            .create()
            .name(name)
            .email(email)
            .ptype(Type::List)
            .members(members.into())
            .create_id()
            .unwrap();
        request
            .send_single::<PrincipalSetResponse>()
            .await?
            .created(&id)
    }

    pub async fn group_create(
        &mut self,
        email: impl Into<String>,
        name: impl Into<String>,
        members: impl IntoIterator<Item = impl Into<String>>,
    ) -> crate::Result<Principal> {
        let mut request = self.build();
        let id = request
            .set_principal()
            .create()
            .name(name)
            .email(email)
            .ptype(Type::Group)
            .members(members.into())
            .create_id()
            .unwrap();
        request
            .send_single::<PrincipalSetResponse>()
            .await?
            .created(&id)
    }

    pub async fn principal_set_name(
        &mut self,
        id: &str,
        name: impl Into<String>,
    ) -> crate::Result<Option<Principal>> {
        let mut request = self.build();
        request.set_principal().update(id).name(name);
        request
            .send_single::<PrincipalSetResponse>()
            .await?
            .updated(id)
    }

    pub async fn principal_set_secret(
        &mut self,
        id: &str,
        secret: impl Into<String>,
    ) -> crate::Result<Option<Principal>> {
        let mut request = self.build();
        request.set_principal().update(id).secret(secret);
        request
            .send_single::<PrincipalSetResponse>()
            .await?
            .updated(id)
    }

    pub async fn principal_set_email(
        &mut self,
        id: &str,
        email: impl Into<String>,
    ) -> crate::Result<Option<Principal>> {
        let mut request = self.build();
        request.set_principal().update(id).email(email);
        request
            .send_single::<PrincipalSetResponse>()
            .await?
            .updated(id)
    }

    pub async fn principal_set_timezone(
        &mut self,
        id: &str,
        timezone: Option<impl Into<String>>,
    ) -> crate::Result<Option<Principal>> {
        let mut request = self.build();
        request.set_principal().update(id).timezone(timezone);
        request
            .send_single::<PrincipalSetResponse>()
            .await?
            .updated(id)
    }

    pub async fn principal_set_members(
        &mut self,
        id: &str,
        members: Option<impl IntoIterator<Item = impl Into<String>>>,
    ) -> crate::Result<Option<Principal>> {
        let mut request = self.build();
        request.set_principal().update(id).members(members);
        request
            .send_single::<PrincipalSetResponse>()
            .await?
            .updated(id)
    }

    pub async fn principal_set_aliases(
        &mut self,
        id: &str,
        aliases: Option<impl IntoIterator<Item = impl Into<String>>>,
    ) -> crate::Result<Option<Principal>> {
        let mut request = self.build();
        request.set_principal().update(id).aliases(aliases);
        request
            .send_single::<PrincipalSetResponse>()
            .await?
            .updated(id)
    }

    pub async fn principal_set_capabilities(
        &mut self,
        id: &str,
        capabilities: Option<impl IntoIterator<Item = impl Into<String>>>,
    ) -> crate::Result<Option<Principal>> {
        let mut request = self.build();
        request
            .set_principal()
            .update(id)
            .capabilities(capabilities);
        request
            .send_single::<PrincipalSetResponse>()
            .await?
            .updated(id)
    }

    pub async fn principal_destroy(&mut self, id: &str) -> crate::Result<()> {
        let mut request = self.build();
        request.set_principal().destroy([id]).arguments();
        request
            .send_single::<PrincipalSetResponse>()
            .await?
            .destroyed(id)
    }

    pub async fn principal_get(
        &mut self,
        id: &str,
        properties: Option<Vec<Property>>,
    ) -> crate::Result<Option<Principal>> {
        let mut request = self.build();
        let get_request = request.get_principal().ids([id]);
        if let Some(properties) = properties {
            get_request.properties(properties.into_iter());
        }
        request
            .send_single::<PrincipalGetResponse>()
            .await
            .map(|mut r| r.unwrap_list().pop())
    }

    pub async fn principal_query(
        &mut self,
        filter: Option<impl Into<Filter<super::query::Filter>>>,
        sort: Option<impl IntoIterator<Item = Comparator<super::query::Comparator>>>,
    ) -> crate::Result<QueryResponse> {
        let mut request = self.build();
        let query_request = request.query_principal();
        if let Some(filter) = filter {
            query_request.filter(filter);
        }
        if let Some(sort) = sort {
            query_request.sort(sort.into_iter());
        }
        request.send_single::<QueryResponse>().await
    }

    pub async fn principal_changes(
        &mut self,
        since_state: impl Into<String>,
        max_changes: usize,
    ) -> crate::Result<ChangesResponse<Principal<Get>>> {
        let mut request = self.build();
        request
            .changes_principal(since_state)
            .max_changes(max_changes);
        request.send_single().await
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