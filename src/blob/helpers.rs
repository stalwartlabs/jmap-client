use crate::{
    core::request::{Arguments, Request},
    Method,
};

use super::copy::{CopyBlobRequest, CopyBlobResponse};

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
