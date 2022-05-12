use crate::{
    client::Client,
    core::{
        query::{Comparator, Filter, QueryResponse},
        response::{MailboxGetResponse, MailboxSetResponse},
        set::Create,
    },
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
        sort: Option<Vec<Comparator<super::query::Comparator>>>,
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
}
