use serde::Serialize;

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: String,
    pub username: String,
}
