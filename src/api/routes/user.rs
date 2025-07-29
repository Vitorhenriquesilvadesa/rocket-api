use std::sync::Arc;

use rocket::{self, Route, State, delete, get, http::Status, post, put, serde::json::Json};
use tracing::{debug, error, info, instrument};

use crate::{
    api::{
        requests::{user_reqs::CreateUserRequest, PageConfig},
        responses::user::{CreateUserResponse, UserDTO},
    },
    core::user::{dto::UpdateUser, model::User, service::UserService},
};

pub fn routes() -> Vec<Route> {
    rocket::routes![create_user, get_all_users, delete_user, update_user]
}

#[instrument(
    name = "create_user_request",
    skip(new_user, user_service),
    fields(user_email = %new_user.email, user_username = %new_user.username)
)]
#[post("/users", data = "<new_user>")]
pub async fn create_user(
    new_user: Json<CreateUserRequest>,
    user_service: &State<Arc<UserService>>,
) -> Result<Json<CreateUserResponse>, Status> {
    info!("Initializing new user creation");

    user_service
        .create_user(
            new_user.username.clone(),
            new_user.email.clone(),
            new_user.password.clone(),
            new_user.roles.clone(),
        )
        .await
        .map(|user| {
            info!(user_id = %user.id, "User created successful.");

            Json(CreateUserResponse {
                id: user.id,
                username: user.username,
            })
        })
        .map_err(|e| {
            error!("Failed to create user: {:?}", e);

            Status::InternalServerError
        })
}

#[instrument(
    name = "get_all_users", 
    skip(user_service, spec), 
    fields(page = %spec.page.unwrap_or(1), per_page = %spec.per_page.unwrap_or(10))
)]
#[get("/users?<spec..>")]
async fn get_all_users(
    spec: PageConfig,
    user_service: &State<Arc<UserService>>,
) -> Result<Json<Vec<UserDTO>>, Status> {
    let users = user_service
        .list_users(spec)
        .await
        .map_err(|_| {
            error!("Error to get users.");
            Status::NotFound})?.into_iter().map(|e| UserDTO {
                email: e.email,
                username: e.username,
                roles: e.roles,

            }).collect();

    debug!("Successful to get users.");
    Ok(Json(users))
}

#[instrument(name="delete_user", skip(user_service), fields(id = id))]
#[delete("/users?<id>")]
async fn delete_user(id: String, user_service: &State<Arc<UserService>>) -> Status {
    user_service.delete_user(id).await.map(|_| Status::NoContent).unwrap_or(Status::NotFound)
}

#[instrument(name="update_user", skip(user_service), fields(username = user_data.username, email = user_data.email))]
#[put("/users?<id>", data = "<user_data>")]
async fn update_user(
    id: String,
    user_data: Json<UpdateUser>,
    user_service: &State<Arc<UserService>>,
) -> Result<Json<User>, Status> {
    let user = user_service
        .update_user(id, user_data.into_inner())
        .await
        .map_err(|_| Status::NotFound)?;

    Ok(Json(user))
}
