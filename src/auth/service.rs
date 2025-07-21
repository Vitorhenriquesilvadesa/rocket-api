use std::sync::Arc;

use jsonwebtoken::{EncodingKey, Header, encode};
use rocket::time::UtcDateTime;

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
        let now = UtcDateTime::now().unix_timestamp();
        let exp = now + self.jwt.expiration;

        let claims = Claims {
            exp: exp as usize,
            sub: user.id.clone(),
            roles: user.roles.clone(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt.secret.as_bytes()),
        )?;

        Ok(token)
    }

    pub async fn validate_token(&self, token: &str) -> Result<Claims, JwtAuthenticationError> {
        let claims = validate_jwt(token, &self.jwt)?;

        match self.user_service.find_by_id(claims.sub.clone()).await {
            Some(_) => Ok(claims),
            None => Err(JwtAuthenticationError::Unauthorized),
        }
    }
}
