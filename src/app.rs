use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;

use config::ConfigError;
use rocket::Build;
use rocket::Config;
use rocket::Rocket;
use rocket_cors::AllowedHeaders;
use rocket_cors::AllowedOrigins;
use rocket_cors::Method;
use thiserror::Error;

use crate::api::routes::get_routes;
use crate::auth::service::AuthService;
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

    #[error("Failed to create CORS configuration")]
    CorsConfiguration(String),
}

pub async fn build_app() -> Result<AppRocket, ApplicationError> {
    let cfg = load_settings();

    let cfg = match cfg {
        Err(e) => return Err(ApplicationError::ConfigurationParsing(e)),
        Ok(c) => c,
    };

    let database_conn = create_surreal_client(&cfg.surrealdb).await?;
    let user_repo = SurrealUserRepository::new(Arc::clone(&database_conn));
    let user_service = Arc::new(UserService::new(Arc::new(user_repo)));

    let auth_service = Arc::new(AuthService::new(Arc::clone(&user_service), cfg.jwt.clone()));

    let allowed_origins = AllowedOrigins::some_exact(&cfg.server.allowed_origins);
    let allowed_headers = AllowedHeaders::some(&[
        "Authorization",
        "Accept",
        "Content-Type",
        "X-Requested-With",
    ]);

    let allowed_methods: HashSet<Method> = cfg
        .server
        .allowed_methods
        .iter()
        .map(|s| Method::from_str(s))
        .collect::<Result<HashSet<Method>, _>>()
        .map_err(|e| {
            ApplicationError::CorsConfiguration(format!("Invalid HTTP method in config: {:?}", e))
        })?;

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_headers,
        allow_credentials: true,
        allowed_methods,
        ..Default::default()
    }
    .to_cors()
    .map_err(|e| {
        ApplicationError::CorsConfiguration(format!("CORS config error: {:?}", e.to_string()))
    })?;

    Ok(rocket::custom(Config {
        port: cfg.server.port,
        address: cfg.server.address,
        ..Default::default()
    })
    .manage(Arc::clone(&user_service))
    .manage(Arc::clone(&auth_service))
    .manage(cfg)
    .mount("/api", get_routes())
    .attach(cors))
}
