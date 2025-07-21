use rocket::async_trait;
use thiserror::Error;

use crate::{
    api::requests::PageConfig,
    core::user::{
        dto::{NewUser, UpdateUser},
        model::User,
    },
};

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("User not found")]
    NotFound,

    #[error("Database connection error")]
    DatabaseError(String),

    #[error("Unknown error")]
    Unknown,

    #[error("Query failed: {0}")]
    QueryFailed(String),
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_by_id(&self, id: String) -> Option<User>;
    async fn get_by_email(&self, username: &str) -> Option<User>;
    async fn create(&self, user: NewUser) -> Result<User, UserRepositoryError>;
    async fn update(&self, id: String, data: UpdateUser) -> Result<User, UserRepositoryError>;
    async fn delete(&self, id: String) -> Result<(), UserRepositoryError>;
    async fn list(&self, spec: PageConfig) -> Result<Vec<User>, UserRepositoryError>;
}
