use jsonwebtoken::{DecodingKey, Validation, decode, errors::ErrorKind};
use rocket::{Request, async_trait, http::Status, request::FromRequest};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    api::middleware::Middleware,
    config::settings::{JwtSettings, Settings},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
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
        let settings = &request
            .rocket()
            .state::<Settings>()
            .ok_or((
                Status::InternalServerError,
                JwtAuthenticationError::Unauthorized,
            ))?
            .jwt;

        let auth_header = request.headers().get_one("Authorization");

        if let Some(auth_header) = auth_header {
            if let Some(token) = auth_header.strip_prefix("Bearer ") {
                match validate_jwt(token, settings) {
                    Ok(claims) => Ok(Self(claims)),
                    Err(e) => Err((Status::Unauthorized, e)),
                }
            } else {
                return Err((Status::Unauthorized, JwtAuthenticationError::MissingToken));
            }
        } else {
            return Err((Status::Unauthorized, JwtAuthenticationError::MissingToken));
        }
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
