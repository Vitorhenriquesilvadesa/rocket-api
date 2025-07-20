use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SurrealDbConfig {
    pub host: String,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub surrealdb: SurrealDbConfig,
}
