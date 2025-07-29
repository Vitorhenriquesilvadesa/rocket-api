use serde::Serialize;

use crate::auth::roles::Role;

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: String,
    pub username: String,
}

#[derive(Serialize)]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub roles: Vec<Role>,
}
