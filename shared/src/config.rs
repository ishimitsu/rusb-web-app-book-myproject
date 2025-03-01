use anyhow::{Ok, Result};

pub struct AppConfig {
    pub database: DatabaseConfig,
}

impl AppConfig {
    /// Get configuration due to connect DataBase, from environment variables.
    pub fn new() -> Result<Self> {
        let database = DatabaseConfig {
            host: std::env::var("DATABASE_HOST")?,
            port: std::env::var("DATABASE_PORT")?.parse()?,
            username: std::env::var("DATABASE_USERNAME")?,
            password: std::env::var("DATABASE_PASSWORD")?,
            database: std::env::var("DATABASE_NAME")?,
        };
        Ok(Self { database })
    }
}

/// Configuration to access DataBase.
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}
