use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub surrealdb: SurrealDbConfig,
    pub jwt: JwtSettings,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SurrealDbConfig {
    pub host: String,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtSettings {
    pub secret: String,
    pub expiration: i64,
}
