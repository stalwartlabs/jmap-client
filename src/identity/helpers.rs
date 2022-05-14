use crate::{
    core::{
        get::GetRequest,
        request::{Arguments, Request},
        response::{IdentityGetResponse, IdentitySetResponse},
        set::SetRequest,
    },
    Method, Set,
};

use super::Identity;

impl Request<'_> {
    pub fn get_identity(&mut self) -> &mut GetRequest<super::Property, ()> {
        self.add_method_call(
            Method::GetIdentity,
            Arguments::identity_get(self.params(Method::GetIdentity)),
        )
        .identity_get_mut()
    }

    pub async fn send_get_identity(self) -> crate::Result<IdentityGetResponse> {
        self.send_single().await
    }

    pub fn set_identity(&mut self) -> &mut SetRequest<Identity<Set>, ()> {
        self.add_method_call(
            Method::SetIdentity,
            Arguments::identity_set(self.params(Method::SetIdentity)),
        )
        .identity_set_mut()
    }

    pub async fn send_set_identity(self) -> crate::Result<IdentitySetResponse> {
        self.send_single().await
    }
}
