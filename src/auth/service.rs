use std::sync::Arc;

use jsonwebtoken::{EncodingKey, Header, encode};
use rocket::time::{Duration, UtcDateTime};

use crate::{
    auth::jwt::{Claims, JwtAuthenticationError, validate_jwt},
    config::settings::JwtSettings,
    core::user::{model::User, service::UserService},
};

pub struct AuthService {
    user_service: Arc<UserService>,
    jwt: JwtSettings,
}

impl AuthService {
    pub fn new(user_service: Arc<UserService>, jwt: JwtSettings) -> Self {
        Self { user_service, jwt }
    }

    pub async fn login(&self, email: String, password: String) -> Result<String, ()> {
        let user = self
            .user_service
            .verify_user(email, password)
            .await
            .map_err(|_| ())?;

        Ok(self.generate_jwt(&user).map_err(|_| ())?)
    }

    pub fn generate_jwt(&self, user: &User) -> Result<String, jsonwebtoken::errors::Error> {
        let now = UtcDateTime::now().millisecond();
        let exp = now as i128 + Duration::minutes(self.jwt.expiration_millis).whole_milliseconds();

        let claims = Claims {
            exp: exp as usize,
            sub: user.id.clone(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt.secret.as_bytes()),
        )?;

        Ok(token)
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, JwtAuthenticationError> {
        validate_jwt(token, &self.jwt)
    }
}
