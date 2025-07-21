use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginResponse {
    pub token: String,
}
