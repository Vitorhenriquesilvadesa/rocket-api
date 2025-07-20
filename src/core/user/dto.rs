use serde::{Deserialize, Serialize};

use crate::core::user::model::{Password, PasswordHash, UserId, Username};

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub username: Username,
    pub password: PasswordHash,
}

pub struct UpdateUser {
    pub username: Option<Username>,
    pub password: Option<Password>,
}

pub struct DeleteUser {
    pub id: UserId,
}

pub struct ListUsers {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}
