use crate::{
    client::Client,
    core::request::{Arguments, Request},
    Method,
};

use super::copy::{CopyBlobRequest, CopyBlobResponse};

impl Client {
    pub async fn blob_copy(
        &self,
        from_account_id: impl Into<String>,
        blob_id: impl Into<String>,
    ) -> crate::Result<String> {
        let blob_id = blob_id.into();
        let mut request = self.build();
        request.copy_blob(from_account_id).blob_id(&blob_id);
        request
            .send_single::<CopyBlobResponse>()
            .await?
            .copied(&blob_id)
    }
}

impl Request<'_> {
    pub fn copy_blob(&mut self, from_account_id: impl Into<String>) -> &mut CopyBlobRequest {
        self.add_method_call(
            Method::CopyBlob,
            Arguments::blob_copy(self.params(Method::CopyBlob), from_account_id.into()),
        )
        .blob_copy_mut()
    }

    pub async fn send_copy_blob(self) -> crate::Result<CopyBlobResponse> {
        self.send_single().await
    }
}
