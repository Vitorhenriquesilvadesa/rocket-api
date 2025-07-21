use std::net::IpAddr;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub surrealdb: SurrealDbConfig,
    pub server: ServerConfig,
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
pub struct ServerConfig {
    pub port: u16,
    pub address: IpAddr,
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtSettings {
    pub secret: String,
    pub expiration: i64,
}
