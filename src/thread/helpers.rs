use crate::{client::Client, core::response::ThreadGetResponse};

use super::Thread;

impl Client {
    pub async fn thread_get(&mut self, id: &str) -> crate::Result<Option<Thread>> {
        let mut request = self.build();
        request.get_thread().ids([id]);
        request
            .send_single::<ThreadGetResponse>()
            .await
            .map(|mut r| r.unwrap_list().pop())
    }
}
