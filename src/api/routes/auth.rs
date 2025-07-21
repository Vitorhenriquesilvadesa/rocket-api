use std::sync::Arc;

use rocket::{Route, State, get, http::Status, post, serde::json::Json};

use crate::{
    api::{
        middleware::MiddlewareGuard,
        requests::auth_reqs::{LoginRequest, LoginResponse},
    },
    auth::{jwt::JwtAuthentication, service::AuthService},
};

pub fn routes() -> Vec<Route> {
    rocket::routes![login, me]
}

#[post("/auth", data = "<credentials>")]
async fn login(
    credentials: Json<LoginRequest>,
    auth_service: &State<Arc<AuthService>>,
) -> Result<Json<LoginResponse>, Status> {
    let token = auth_service
        .login(credentials.email.clone(), credentials.password.clone())
        .await
        .map_err(|_| Status::Unauthorized)?;

    Ok(Json(LoginResponse { token }))
}

#[get("/me")]
async fn me(auth: MiddlewareGuard<JwtAuthentication>) -> String {
    format!("Usuário autenticado: {:?}", auth.0)
}
