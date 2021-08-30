use yew::callback::Callback;
use yew::services::fetch::FetchTask;

use super::Requests;
use crate::error::Error;
use crate::types::*;

#[derive(Default, Debug)]
pub struct Auth {
    requests: Requests,
}

impl Auth {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    pub fn login(
        &mut self,
        login_info: LoginInfoWrapper,
        callback: Callback<Result<UserInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests.post::<LoginInfoWrapper, UserInfoWrapper>(
            "/accounts/login".to_string(),
            login_info,
            callback,
        )
    }
}
