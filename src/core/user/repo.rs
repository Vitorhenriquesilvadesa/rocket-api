use rocket::async_trait;
use thiserror::Error;

use crate::core::user::{
    dto::{ListUsers, NewUser},
    model::{User, UserId},
};

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("User not found")]
    NotFound,

    #[error("Database connection error")]
    DatabaseError(String),

    #[error("Unknown error")]
    Unknown,

    #[error("Query failed")]
    QueryFailed,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_by_id(&self, id: UserId) -> Option<User>;
    async fn create(&self, user: NewUser) -> Result<User, UserRepositoryError>;
    async fn update(&self, user: User) -> Result<User, UserRepositoryError>;
    async fn delete(&self, id: UserId) -> Result<(), UserRepositoryError>;
    async fn list(&self, spec: ListUsers) -> Result<Vec<User>, UserRepositoryError>;
}
