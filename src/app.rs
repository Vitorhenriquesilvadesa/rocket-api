use std::sync::Arc;

use config::ConfigError;
use rocket::Build;
use rocket::Config;
use rocket::Rocket;
use thiserror::Error;

use crate::api::routes::get_routes;
use crate::api::routes::user::*;
use crate::config::load_settings;
use crate::core::user::service::UserService;
use crate::infra::db::connection::create_surreal_client;
use crate::infra::db::user_repo::SurrealUserRepository;

pub type AppRocket = Rocket<Build>;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Failed to connect database: {0}")]
    DatabaseConnection(String),
    #[error("Failed to parse configuration file")]
    ConfigurationParsing(ConfigError),
}

pub async fn build_app() -> Result<AppRocket, ApplicationError> {
    let cfg = load_settings();

    let cfg = match cfg {
        Err(e) => return Err(ApplicationError::ConfigurationParsing(e)),
        Ok(c) => c,
    };

    let database_conn = create_surreal_client(&cfg.surrealdb).await?;
    let user_repo = SurrealUserRepository::new(Arc::clone(&database_conn));
    let user_service = UserService::new(Arc::new(user_repo));

    Ok(rocket::custom(Config {
        port: 5173,
        ..Default::default()
    })
    .manage(user_service)
    .mount("/api", get_routes()))
}
