use serde::{Deserialize, Serialize};

use crate::auth::roles::Role;

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub roles: Vec<Role>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteUser {
    pub id: String,
}

pub struct ListUsers {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}
