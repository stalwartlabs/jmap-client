use crate::{
    client::Client,
    core::{
        get::GetRequest,
        request::{Arguments, Request},
        response::{PushSubscriptionGetResponse, PushSubscriptionSetResponse},
        set::{SetObject, SetRequest},
    },
    Method, Set, TypeState,
};

use super::{Keys, PushSubscription};

impl Client {
    pub async fn push_subscription_create(
        &self,
        device_client_id: impl Into<String>,
        url: impl Into<String>,
        keys: Option<Keys>,
    ) -> crate::Result<PushSubscription> {
        let mut request = self.build();
        let create_req = request
            .set_push_subscription()
            .create()
            .device_client_id(device_client_id)
            .url(url);

        if let Some(keys) = keys {
            create_req.keys(keys);
        }

        let id = create_req.create_id().unwrap();
        request
            .send_single::<PushSubscriptionSetResponse>()
            .await?
            .created(&id)
    }

    pub async fn push_subscription_verify(
        &self,
        id: &str,
        verification_code: impl Into<String>,
    ) -> crate::Result<Option<PushSubscription>> {
        let mut request = self.build();
        request
            .set_push_subscription()
            .update(id)
            .verification_code(verification_code);
        request
            .send_single::<PushSubscriptionSetResponse>()
            .await?
            .updated(id)
    }

    pub async fn push_subscription_update_types(
        &self,
        id: &str,
        types: Option<impl IntoIterator<Item = TypeState>>,
    ) -> crate::Result<Option<PushSubscription>> {
        let mut request = self.build();
        request.set_push_subscription().update(id).types(types);
        request
            .send_single::<PushSubscriptionSetResponse>()
            .await?
            .updated(id)
    }

    pub async fn push_subscription_destroy(&self, id: &str) -> crate::Result<()> {
        let mut request = self.build();
        request.set_push_subscription().destroy([id]);
        request
            .send_single::<PushSubscriptionSetResponse>()
            .await?
            .destroyed(id)
    }
}

impl Request<'_> {
    pub fn get_push_subscription(&mut self) -> &mut GetRequest<PushSubscription<Set>> {
        self.add_method_call(
            Method::GetPushSubscription,
            Arguments::push_get(self.params(Method::GetPushSubscription)),
        )
        .push_get_mut()
    }

    pub async fn send_get_push_subscription(self) -> crate::Result<PushSubscriptionGetResponse> {
        self.send_single().await
    }

    pub fn set_push_subscription(&mut self) -> &mut SetRequest<PushSubscription<Set>> {
        self.add_method_call(
            Method::SetPushSubscription,
            Arguments::push_set(self.params(Method::SetPushSubscription)),
        )
        .push_set_mut()
    }

    pub async fn send_set_push_subscription(self) -> crate::Result<PushSubscriptionSetResponse> {
        self.send_single().await
    }
}
