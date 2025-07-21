use std::{collections::HashMap, sync::Arc};

use rocket::async_trait;
use surrealdb::{Surreal, engine::remote::ws::Client};

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
        match self.client.select(id.to_string()).await {
            Err(_) => None,
            Ok(mut users) => users.pop(),
        }
    }

    async fn get_by_email(&self, email: &str) -> Option<User> {
        let query = format!("SELECT * FROM users WHERE email = '{}' LIMIT 1", email);

        let mut response = match self.client.query(query).await {
            Ok(resp) => resp,
            Err(_) => return None,
        };

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

        let created_user = response.take();

        Ok(created_user.unwrap())
    }

    async fn update(&self, id: String, data: UpdateUser) -> Result<User, UserRepositoryError> {
        let mut fields = HashMap::new();

        if let Some(name) = data.username {
            fields.insert("username", surrealdb::sql::Value::from(name.to_string()));
        }

        if let Some(password) = data.password {
            let password = PasswordHash::raw(password.into())
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

        let user = match user {
            Some(v) => v,
            None => return Err(UserRepositoryError::NotFound),
        };

        Ok(user)
    }

    async fn delete(&self, id: String) -> Result<(), UserRepositoryError> {
        let response: Option<User> = self
            .client
            .delete(("users", id.to_string()))
            .await
            .map_err(|e| UserRepositoryError::QueryFailed(e.to_string()))?;

        match response {
            None => Err(UserRepositoryError::NotFound),
            Some(_) => Ok(()),
        }
    }

    async fn list(&self, spec: PageConfig) -> Result<Vec<User>, UserRepositoryError> {
        let page = spec.page.unwrap_or(1);
        let per_page = spec.per_page.unwrap_or(10);
        let start = (page - 1) * per_page;

        let query = format!("SELECT * FROM users START {} LIMIT {}", start, per_page);

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
