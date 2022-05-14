use crate::{
    core::{
        get::GetRequest,
        request::{Arguments, Request},
        response::{VacationResponseGetResponse, VacationResponseSetResponse},
        set::SetRequest,
    },
    Method, Set, URI,
};

use super::VacationResponse;

impl Request<'_> {
    pub fn get_vacation_response(&mut self) -> &mut GetRequest<super::Property, ()> {
        self.add_capability(URI::VacationResponse);
        self.add_method_call(
            Method::GetVacationResponse,
            Arguments::vacation_response_get(self.params(Method::GetVacationResponse)),
        )
        .vacation_response_get_mut()
    }

    pub async fn send_get_vacation_response(self) -> crate::Result<VacationResponseGetResponse> {
        self.send_single().await
    }

    pub fn set_vacation_response(&mut self) -> &mut SetRequest<VacationResponse<Set>, ()> {
        self.add_capability(URI::VacationResponse);
        self.add_method_call(
            Method::SetVacationResponse,
            Arguments::vacation_response_set(self.params(Method::GetVacationResponse)),
        )
        .vacation_response_set_mut()
    }

    pub async fn send_set_vacation_response(self) -> crate::Result<VacationResponseSetResponse> {
        self.send_single().await
    }
}
