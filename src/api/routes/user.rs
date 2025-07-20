use rocket::{
    self, Route, State, get, http::Status, post, response::status::Custom, serde::json::Json,
};

use crate::{
    api::{
        requests::{Pageable, user_reqs::CreateUserRequest},
        responses::user::CreateUserResponse,
    },
    core::user::{dto::ListUsers, model::User, service::UserService},
};

pub fn routes() -> Vec<Route> {
    rocket::routes![create_user, get_all_users]
}

#[post("/users", data = "<new_user>")]
pub async fn create_user(
    new_user: Json<CreateUserRequest>,
    user_service: &State<UserService>,
) -> Result<Json<CreateUserResponse>, Custom<String>> {
    user_service
        .create_user(new_user.username.clone(), new_user.password.clone())
        .await
        .map(|user| {
            Json(CreateUserResponse {
                id: user.id,
                username: user.username,
            })
        })
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))
}

#[get("/users", data = "<spec>")]
async fn get_all_users(
    spec: Json<Pageable>,
    user_service: &State<UserService>,
) -> Result<Json<Vec<User>>, Custom<String>> {
    let users = user_service
        .list_users(ListUsers {
            page: None,
            per_page: None,
        })
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json(users))
}
