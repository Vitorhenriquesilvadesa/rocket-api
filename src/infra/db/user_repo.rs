use std::sync::Arc;

use rocket::async_trait;
use surrealdb::{Surreal, engine::remote::ws::Client};

use crate::core::user::{
    dto::{ListUsers, NewUser},
    model::{User, UserId},
    repo::{UserRepository, UserRepositoryError},
};

pub struct SurrealUserRepository {
    client: Arc<Surreal<Client>>,
}

impl SurrealUserRepository {
    pub fn new(client: Arc<Surreal<Client>>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl UserRepository for SurrealUserRepository {
    async fn get_by_id(&self, id: UserId) -> Option<User> {
        match self.client.select(id.to_string()).await {
            Err(_) => None,
            Ok(mut users) => users.pop(),
        }
    }

    async fn create(&self, new_user: NewUser) -> Result<User, UserRepositoryError> {
        let mut response: Option<User> = self
            .client
            .create("users")
            .content(new_user)
            .await
            .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        let created_user = response.take();

        Ok(created_user.unwrap())
    }

    async fn update(&self, user: User) -> Result<User, UserRepositoryError> {
        todo!()
    }

    async fn delete(&self, id: UserId) -> Result<(), UserRepositoryError> {
        todo!()
    }

    async fn list(&self, spec: ListUsers) -> Result<Vec<User>, UserRepositoryError> {
        let page = spec.page.unwrap_or(1);
        let per_page = spec.per_page.unwrap_or(10);
        let start = (page - 1) * per_page;

        let query = format!("SELECT * FROM users START {} LIMIT {}", start, per_page);

        let mut response = self
            .client
            .query(query)
            .await
            .map_err(|_| UserRepositoryError::QueryFailed)?;
        let users: Vec<User> = response
            .take(0)
            .map_err(|e| UserRepositoryError::NotFound)?;

        Ok(users)
    }
}
