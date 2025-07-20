use std::sync::Arc;

use crate::core::user::{
    dto::{ListUsers, NewUser},
    error::UserServiceError,
    model::{PasswordHash, User, Username},
    repo::{UserRepository, UserRepositoryError},
};

pub struct UserService {
    repo: Arc<dyn UserRepository + Send + Sync>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { repo }
    }

    pub async fn create_user(
        &self,
        username: String,
        raw_password: String,
    ) -> Result<User, UserServiceError> {
        let username = Username::new(username);
        let password_hash = PasswordHash::raw(raw_password)
            .map_err(|e| UserServiceError::PasswordHashError(e.to_string()))?;

        println!("{}", password_hash.as_str());

        let new_user = NewUser {
            username,
            password: password_hash,
        };

        let user = self.repo.create(new_user).await.map_err(|e| e.into())?;

        Ok(user)
    }

    pub async fn list_users(&self, spec: ListUsers) -> Result<Vec<User>, UserServiceError> {
        let users = self.repo.list(spec).await.map_err(|e| e.into())?;
        Ok(users)
    }
}

impl Into<UserServiceError> for UserRepositoryError {
    fn into(self) -> UserServiceError {
        match self {
            UserRepositoryError::DatabaseError(err) => UserServiceError::RepositoryError(err),
            UserRepositoryError::NotFound => UserServiceError::UserNotFound,
            UserRepositoryError::Unknown => UserServiceError::Unknown,
            UserRepositoryError::QueryFailed => UserServiceError::RepositoryError(self.to_string()),
        }
    }
}
