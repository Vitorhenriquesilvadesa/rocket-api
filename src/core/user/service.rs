use std::sync::Arc;

use crate::{
    api::requests::PageConfig,
    core::user::{
        dto::{NewUser, UpdateUser},
        error::UserServiceError,
        model::{PasswordHash, User},
        repo::{UserRepository, UserRepositoryError},
    },
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
        email: String,
        raw_password: String,
    ) -> Result<User, UserServiceError> {
        let password_hash = PasswordHash::raw(raw_password)
            .map_err(|e| UserServiceError::PasswordHashError(e.to_string()))?;

        let new_user = NewUser {
            username,
            email,
            password: password_hash.as_str().to_string(),
        };

        let user = self.repo.create(new_user).await.map_err(|e| e.into())?;

        Ok(user)
    }

    pub async fn list_users(&self, spec: PageConfig) -> Result<Vec<User>, UserServiceError> {
        let users = self.repo.list(spec).await.map_err(|e| e.into())?;
        Ok(users)
    }

    pub async fn delete_user(&self, id: String) -> Result<(), UserServiceError> {
        self.repo.delete(id).await.map_err(|e| e.into())
    }

    pub async fn update_user(
        &self,
        id: String,
        data: UpdateUser,
    ) -> Result<User, UserServiceError> {
        self.repo.update(id, data).await.map_err(|e| e.into())
    }

    pub async fn verify_user(
        &self,
        email: String,
        raw_password: String,
    ) -> Result<User, UserServiceError> {
        let user = match self.repo.get_by_email(&email).await {
            Some(user) => user,
            None => return Err(UserServiceError::UserNotFound),
        };

        if !user.password.verify(&raw_password) {
            return Err(UserServiceError::UserNotFound);
        }

        Ok(user)
    }
}

impl Into<UserServiceError> for UserRepositoryError {
    fn into(self) -> UserServiceError {
        match self {
            UserRepositoryError::DatabaseError(err) => UserServiceError::RepositoryError(err),
            UserRepositoryError::NotFound => UserServiceError::UserNotFound,
            UserRepositoryError::Unknown => UserServiceError::Unknown,
            UserRepositoryError::QueryFailed(reason) => UserServiceError::RepositoryError(reason),
        }
    }
}
