use crate::{
    client::Client,
    core::{
        changes::{ChangesRequest, ChangesResponse},
        get::{GetObject, GetRequest},
        request::{Arguments, Request},
        response::ThreadGetResponse,
    },
    Method,
};

use super::Thread;

impl Client {
    pub async fn thread_get(&self, id: &str) -> crate::Result<Option<Thread>> {
        let mut request = self.build();
        request.get_thread().ids([id]);
        request
            .send_single::<ThreadGetResponse>()
            .await
            .map(|mut r| r.unwrap_list().pop())
    }
}

impl Request<'_> {
    pub fn get_thread(&mut self) -> &mut GetRequest<Thread> {
        self.add_method_call(
            Method::GetThread,
            Arguments::thread_get(self.params(Method::GetThread)),
        )
        .thread_get_mut()
    }

    pub async fn send_get_thread(self) -> crate::Result<ThreadGetResponse> {
        self.send_single().await
    }

    pub fn changes_thread(&mut self, since_state: impl Into<String>) -> &mut ChangesRequest {
        self.add_method_call(
            Method::ChangesThread,
            Arguments::changes(self.params(Method::ChangesThread), since_state.into()),
        )
        .changes_mut()
    }

    pub async fn send_changes_thread(self) -> crate::Result<ChangesResponse<Thread>> {
        self.send_single().await
    }
}

impl GetObject for Thread {
    type GetArguments = ();
}
