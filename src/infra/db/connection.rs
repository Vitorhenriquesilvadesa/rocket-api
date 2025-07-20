use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;

use crate::app::ApplicationError;
use crate::config::settings::SurrealDbConfig;

pub async fn create_surreal_client(
    cfg: &SurrealDbConfig,
) -> Result<Arc<Surreal<Client>>, ApplicationError> {
    let client = Surreal::new::<Ws>(&cfg.host)
        .await
        .map_err(|e| ApplicationError::DatabaseConnection(e.to_string()))?;

    client
        .signin(Root {
            username: &cfg.username,
            password: &cfg.password,
        })
        .await
        .map_err(|e| ApplicationError::DatabaseConnection(e.to_string()))?;

    client
        .use_ns(&cfg.namespace)
        .use_db(&cfg.database)
        .await
        .map_err(|e| ApplicationError::DatabaseConnection(e.to_string()))?;

    Ok(Arc::new(client))
}
