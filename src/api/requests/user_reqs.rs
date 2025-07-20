use serde::Deserialize;

use crate::{api::requests::Pageable, core::user::model::UserId};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteUserRequest {
    pub id: UserId,
}

#[derive(Debug, Deserialize)]
pub struct ListUsersRequest {
    pub config: Pageable,
}
