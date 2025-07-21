use std::sync::Arc;

use rocket::{self, Route, State, delete, get, http::Status, post, put, serde::json::Json};

use crate::{
    api::{
        requests::{PageConfig, user_reqs::CreateUserRequest},
        responses::user::CreateUserResponse,
    },
    core::user::{dto::UpdateUser, model::User, service::UserService},
};

pub fn routes() -> Vec<Route> {
    rocket::routes![create_user, get_all_users, delete_user, update_user]
}

#[post("/users", data = "<new_user>")]
pub async fn create_user(
    new_user: Json<CreateUserRequest>,
    user_service: &State<Arc<UserService>>,
) -> Result<Json<CreateUserResponse>, Status> {
    user_service
        .create_user(
            new_user.username.clone(),
            new_user.email.clone(),
            new_user.password.clone(),
        )
        .await
        .map(|user| {
            Json(CreateUserResponse {
                id: user.id,
                username: user.username,
            })
        })
        .map_err(|e| {
            println!("{:?}", e);
            Status::InternalServerError
        })
}

#[get("/users?<spec..>")]
async fn get_all_users(
    spec: PageConfig,
    user_service: &State<Arc<UserService>>,
) -> Result<Json<Vec<User>>, Status> {
    let users = user_service
        .list_users(spec)
        .await
        .map_err(|_| Status::NotFound)?;

    Ok(Json(users))
}

#[delete("/users?<id>")]
async fn delete_user(id: String, user_service: &State<Arc<UserService>>) -> Status {
    match user_service.delete_user(id).await {
        Err(_) => Status::NotFound,
        Ok(_) => Status::NoContent,
    }
}

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
