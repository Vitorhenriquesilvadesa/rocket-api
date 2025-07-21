use std::marker::PhantomData;

use crate::api::middleware::{Middleware, MiddlewareGuard};
use crate::auth::jwt::JwtAuthentication;
use crate::auth::role_traits::RequiredRole;
use rocket::{Request, async_trait, http::Status};

pub struct RoleAuthorization<T: RequiredRole>(PhantomData<T>);

#[derive(Debug, thiserror::Error)]
pub enum RoleAuthorizationError {
    #[error("Unauthorized role")]
    Unauthorized,
}

#[async_trait]
impl<T: RequiredRole + Send + Sync> Middleware for RoleAuthorization<T> {
    type Error = RoleAuthorizationError;

    async fn from_request(req: &Request<'_>) -> Result<Self, (Status, Self::Error)> {
        let jwt = req
            .guard::<MiddlewareGuard<JwtAuthentication>>()
            .await
            .succeeded()
            .ok_or((Status::Unauthorized, RoleAuthorizationError::Unauthorized))?;

        let role = &jwt.0.0.roles;

        if role.contains(&T::ROLE) {
            Ok(Self(PhantomData))
        } else {
            Err((Status::Forbidden, RoleAuthorizationError::Unauthorized))
        }
    }
}
