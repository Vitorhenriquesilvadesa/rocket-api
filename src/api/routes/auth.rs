use std::sync::Arc;

use rocket::{Route, State, get, http::Status, post, serde::json::Json};
use tracing::{info, instrument};

use crate::{
    api::{
        middleware::MiddlewareGuard,
        requests::auth_reqs::{LoginRequest, LoginResponse},
    },
    auth::{
        jwt::JwtAuthentication, role_middleware::RoleAuthorization, roles::Admin,
        service::AuthService,
    },
};

pub fn routes() -> Vec<Route> {
    rocket::routes![login, me, admin]
}

#[instrument(name = "login_request", skip(auth_service, credentials))]
#[post("/auth", data = "<credentials>")]
async fn login(
    credentials: Json<LoginRequest>,
    auth_service: &State<Arc<AuthService>>,
) -> Result<Json<LoginResponse>, Status> {
    let token = auth_service
        .login(credentials.email.clone(), credentials.password.clone())
        .await
        .map_err(|_| Status::Unauthorized)?;

    info!("Login received with: {} email", credentials.email);
    Ok(Json(LoginResponse { token }))
}

#[get("/me")]
async fn me(auth: MiddlewareGuard<JwtAuthentication>) -> String {
    format!("Usuário autenticado: {:?}", auth.0)
}

#[instrument(name = "admin_test", skip(_jwt, _auth))]
#[get("/admin")]
async fn admin(
    _jwt: MiddlewareGuard<JwtAuthentication>,
    _auth: MiddlewareGuard<RoleAuthorization<Admin>>,
) -> String {
    format!("Usuário autenticado")
}
