use serde::Serialize;

use crate::core::user::model::{UserId, Username};

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: UserId,
    pub username: Username,
}
