use shared::config::DatabaseConfig;
use sqlx::{postgres::PgConnectOptions, PgPool};

/// Transfer `DatabaseConfig` into `PgConnectOptions`.
fn make_pg_connect_options(cfg: &DatabaseConfig) -> PgConnectOptions {
    PgConnectOptions::new()
        .host(&cfg.host)
        .port(cfg.port)
        .username(&cfg.username)
        .password(&cfg.password)
        .database(&cfg.database)
}

#[derive(Clone)]
pub struct ConnectionPool(PgPool);

impl ConnectionPool {
    /// Get reference of sqlx::PgPool.
    pub fn inner_ref(&self) -> &PgPool {
        &self.0
    }
}

/// Create connection pool for PostgreSQL database.
pub fn connect_database_with(config: &DatabaseConfig) -> ConnectionPool {
    ConnectionPool(PgPool::connect_lazy_with(make_pg_connect_options(config)))
}
