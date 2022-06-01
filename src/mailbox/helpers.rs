use crate::{
    client::Client,
    core::{
        changes::{ChangesRequest, ChangesResponse},
        get::GetRequest,
        query::{Comparator, Filter, QueryRequest, QueryResponse},
        query_changes::{QueryChangesRequest, QueryChangesResponse},
        request::{Arguments, Request},
        response::{MailboxGetResponse, MailboxSetResponse},
        set::{SetObject, SetRequest},
    },
    Get, Method, Set,
};

use super::{Mailbox, Property, Role};

impl Client {
    pub async fn mailbox_create(
        &mut self,
        name: impl Into<String>,
        parent_id: Option<impl Into<String>>,
        role: Role,
    ) -> crate::Result<Mailbox> {
        let mut request = self.build();
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
            .created(&id)
    }

    pub async fn mailbox_rename(
        &mut self,
        id: &str,
        name: impl Into<String>,
    ) -> crate::Result<Option<Mailbox>> {
        let mut request = self.build();
        request.set_mailbox().update(id).name(name);
        request
            .send_single::<MailboxSetResponse>()
            .await?
            .updated(id)
    }

    pub async fn mailbox_move(
        &mut self,
        id: &str,
        parent_id: Option<impl Into<String>>,
    ) -> crate::Result<Option<Mailbox>> {
        let mut request = self.build();
        request.set_mailbox().update(id).parent_id(parent_id);
        request
            .send_single::<MailboxSetResponse>()
            .await?
            .updated(id)
    }

    pub async fn mailbox_update_role(
        &mut self,
        id: &str,
        role: Role,
    ) -> crate::Result<Option<Mailbox>> {
        let mut request = self.build();
        request.set_mailbox().update(id).role(role);
        request
            .send_single::<MailboxSetResponse>()
            .await?
            .updated(id)
    }

    pub async fn mailbox_update_sort_order(
        &mut self,
        id: &str,
        sort_order: u32,
    ) -> crate::Result<Option<Mailbox>> {
        let mut request = self.build();
        request.set_mailbox().update(id).sort_order(sort_order);
        request
            .send_single::<MailboxSetResponse>()
            .await?
            .updated(id)
    }

    pub async fn mailbox_destroy(&mut self, id: &str, delete_emails: bool) -> crate::Result<()> {
        let mut request = self.build();
        request
            .set_mailbox()
            .destroy([id])
            .arguments()
            .on_destroy_remove_emails(delete_emails);
        request
            .send_single::<MailboxSetResponse>()
            .await?
            .destroyed(id)
    }

    pub async fn mailbox_get(
        &mut self,
        id: &str,
        properties: Option<Vec<Property>>,
    ) -> crate::Result<Option<Mailbox>> {
        let mut request = self.build();
        let get_request = request.get_mailbox().ids([id]);
        if let Some(properties) = properties {
            get_request.properties(properties.into_iter());
        }
        request
            .send_single::<MailboxGetResponse>()
            .await
            .map(|mut r| r.unwrap_list().pop())
    }

    pub async fn mailbox_query(
        &mut self,
        filter: Option<impl Into<Filter<super::query::Filter>>>,
        sort: Option<impl IntoIterator<Item = Comparator<super::query::Comparator>>>,
    ) -> crate::Result<QueryResponse> {
        let mut request = self.build();
        let query_request = request.query_mailbox();
        if let Some(filter) = filter {
            query_request.filter(filter);
        }
        if let Some(sort) = sort {
            query_request.sort(sort.into_iter());
        }
        request.send_single::<QueryResponse>().await
    }

    pub async fn mailbox_changes(
        &mut self,
        since_state: impl Into<String>,
        max_changes: usize,
    ) -> crate::Result<ChangesResponse<Mailbox<Get>>> {
        let mut request = self.build();
        request
            .changes_mailbox(since_state)
            .max_changes(max_changes);
        request.send_single().await
    }
}

impl Request<'_> {
    pub fn get_mailbox(&mut self) -> &mut GetRequest<Mailbox<Set>> {
        self.add_method_call(
            Method::GetMailbox,
            Arguments::mailbox_get(self.params(Method::GetMailbox)),
        )
        .mailbox_get_mut()
    }

    pub async fn send_get_mailbox(self) -> crate::Result<MailboxGetResponse> {
        self.send_single().await
    }

    pub fn changes_mailbox(&mut self, since_state: impl Into<String>) -> &mut ChangesRequest {
        self.add_method_call(
            Method::ChangesMailbox,
            Arguments::changes(self.params(Method::ChangesMailbox), since_state.into()),
        )
        .changes_mut()
    }

    pub async fn send_changes_mailbox(self) -> crate::Result<ChangesResponse<Mailbox<Get>>> {
        self.send_single().await
    }

    pub fn query_mailbox(&mut self) -> &mut QueryRequest<Mailbox<Set>> {
        self.add_method_call(
            Method::QueryMailbox,
            Arguments::mailbox_query(self.params(Method::QueryMailbox)),
        )
        .mailbox_query_mut()
    }

    pub async fn send_query_mailbox(self) -> crate::Result<QueryResponse> {
        self.send_single().await
    }

    pub fn query_mailbox_changes(
        &mut self,
        since_query_state: impl Into<String>,
    ) -> &mut QueryChangesRequest<Mailbox<Set>> {
        self.add_method_call(
            Method::QueryChangesMailbox,
            Arguments::mailbox_query_changes(
                self.params(Method::QueryChangesMailbox),
                since_query_state.into(),
            ),
        )
        .mailbox_query_changes_mut()
    }

    pub async fn send_query_mailbox_changes(self) -> crate::Result<QueryChangesResponse> {
        self.send_single().await
    }

    pub fn set_mailbox(&mut self) -> &mut SetRequest<Mailbox<Set>> {
        self.add_method_call(
            Method::SetMailbox,
            Arguments::mailbox_set(self.params(Method::SetMailbox)),
        )
        .mailbox_set_mut()
    }

    pub async fn send_set_mailbox(self) -> crate::Result<MailboxSetResponse> {
        self.send_single().await
    }
}
