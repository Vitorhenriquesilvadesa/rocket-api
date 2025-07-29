use std::{collections::HashMap, sync::Arc};

use rocket::async_trait;
use surrealdb::{Surreal, engine::remote::ws::Client};
use tracing::error;

use crate::{
    api::requests::PageConfig,
    core::user::{
        dto::{NewUser, UpdateUser},
        model::{PasswordHash, User},
        repo::{UserRepository, UserRepositoryError},
    },
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
    async fn get_by_id(&self, id: String) -> Option<User> {
        let id = format!("users:{id}");
        let query = format!("SELECT * FROM users WHERE id = {id} LIMIT 1;");

        let mut response = self
            .client
            .query(query)
            .await
            .map_err(|e| UserRepositoryError::QueryFailed(e.to_string()))
            .inspect_err(|e| error!("{:?}", e))
            .ok()?;

        let user: Option<User> = response
            .take(0)
            .map_err(|_| UserRepositoryError::NotFound)
            .ok()?;

        user
    }

    async fn get_by_email(&self, email: &str) -> Option<User> {
        let query = format!("SELECT * FROM users WHERE email = '{email}' LIMIT 1");
        let mut response = self.client.query(query).await.ok()?;
        let users: Vec<User> = response.take(0).ok()?;

        users.into_iter().next()
    }

    async fn create(&self, new_user: NewUser) -> Result<User, UserRepositoryError> {
        let mut response: Option<User> = self
            .client
            .create("users")
            .content(new_user)
            .await
            .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        let created_user = response.take().ok_or(UserRepositoryError::QueryFailed(
            "User creation failed".into(),
        ))?;

        Ok(created_user)
    }

    async fn update(&self, id: String, data: UpdateUser) -> Result<User, UserRepositoryError> {
        let mut fields = HashMap::new();

        if let Some(name) = data.username {
            fields.insert("username", surrealdb::sql::Value::from(name.to_string()));
        }

        if let Some(password) = data.password {
            let password = PasswordHash::raw(password)
                .map_err(|e| UserRepositoryError::QueryFailed(e.to_string()))?;

            let password = password.as_str();

            fields.insert("password", surrealdb::sql::Value::from(password));
        }

        let user: Option<User> = self
            .client
            .update(("users", id.to_string().as_str()))
            .merge(fields)
            .await
            .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        let user = user.ok_or(UserRepositoryError::NotFound)?;

        Ok(user)
    }

    async fn delete(&self, id: String) -> Result<(), UserRepositoryError> {
        self.client
            .delete(("users", id.to_string()))
            .await
            .map_err(|e| UserRepositoryError::QueryFailed(e.to_string()))?
            .ok_or(UserRepositoryError::NotFound)
    }

    async fn list(&self, spec: PageConfig) -> Result<Vec<User>, UserRepositoryError> {
        let page = spec.page.unwrap_or(1);
        let per_page = spec.per_page.unwrap_or(10);
        let start = (page - 1) * per_page;

        let query = format!("SELECT * FROM users START {start} LIMIT {per_page}");

        let mut response = self
            .client
            .query(query)
            .await
            .map_err(|e| UserRepositoryError::QueryFailed(e.to_string()))?;
        let users: Vec<User> = response
            .take(0)
            .map_err(|_| UserRepositoryError::NotFound)?;

        Ok(users)
    }
}
