use std::sync::Arc;

use jsonwebtoken::{DecodingKey, Validation, decode, errors::ErrorKind};
use rocket::{Request, async_trait, http::Status};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    api::middleware::Middleware,
    auth::{roles::Role, service::AuthService},
    config::settings::JwtSettings,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub roles: Vec<Role>,
}

#[derive(Debug)]
pub struct JwtAuthentication(pub Claims);

#[derive(Debug, Error)]
pub enum JwtAuthenticationError {
    #[error("Token has been expired")]
    ExpiredToken,

    #[error("Invalid Token")]
    InvalidToken,

    #[error("Missing Token")]
    MissingToken,

    #[error("User unauthorized")]
    Unauthorized,
}

#[async_trait]
impl Middleware for JwtAuthentication {
    type Error = JwtAuthenticationError;

    async fn from_request(request: &Request<'_>) -> Result<Self, (Status, Self::Error)> {
        let auth_service = request
            .rocket()
            .state::<Arc<AuthService>>()
            .ok_or((Status::Unauthorized, JwtAuthenticationError::Unauthorized))?;

        let token = request
            .headers()
            .get_one("Authorization")
            .and_then(|h| h.strip_prefix("Bearer "))
            .ok_or((Status::Unauthorized, JwtAuthenticationError::MissingToken))?;

        auth_service
            .validate_token(token)
            .await
            .map(Self)
            .map_err(|e| (Status::Unauthorized, e))
    }
}

pub(super) fn validate_jwt(
    token: &str,
    settings: &JwtSettings,
) -> Result<Claims, JwtAuthenticationError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(settings.secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|err| match err.kind() {
        ErrorKind::ExpiredSignature => JwtAuthenticationError::ExpiredToken,
        _ => JwtAuthenticationError::InvalidToken,
    })
}
