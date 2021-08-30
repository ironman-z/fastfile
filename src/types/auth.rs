use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginInfoWrapper {
    pub user: LoginInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UserInfo {
    pub email: String,
    pub token: String,
    pub username: String,
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserInfoWrapper {
    pub user: UserInfo,
}
