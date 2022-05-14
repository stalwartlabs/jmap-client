use crate::{
    core::{
        get::GetRequest,
        request::{Arguments, Request},
        response::{PushSubscriptionGetResponse, PushSubscriptionSetResponse},
        set::SetRequest,
    },
    Method, Set,
};

use super::PushSubscription;

impl Request<'_> {
    pub fn get_push_subscription(&mut self) -> &mut GetRequest<super::Property, ()> {
        self.add_method_call(
            Method::GetPushSubscription,
            Arguments::push_get(self.params(Method::GetPushSubscription)),
        )
        .push_get_mut()
    }

    pub async fn send_get_push_subscription(self) -> crate::Result<PushSubscriptionGetResponse> {
        self.send_single().await
    }

    pub fn set_push_subscription(&mut self) -> &mut SetRequest<PushSubscription<Set>, ()> {
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
