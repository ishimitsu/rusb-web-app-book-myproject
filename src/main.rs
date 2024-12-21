use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use axum::{extract::State, http::StatusCode, routing::get, Router};
use sqlx::{postgres::PgConnectOptions, PgPool};
use tokio::net::TcpListener;

/// Test handler health-check that returns `200 OK` status code.
#[tokio::test]
async fn health_check_works() {
    let status_code = health_check().await;
    assert_eq!(status_code, StatusCode::OK);
}

/// Test handler health-check-db that returns `200 OK` status code.
#[sqlx::test]
async fn health_check_db_works(pool: sqlx::PgPool) {
    let status_code = health_check_db(State(pool)).await;
    assert_eq!(status_code, StatusCode::OK);
}

struct DatabaseConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,
}

/// Implementing `DatabaseConfig` to convert it into `PgConnectOptions`.
impl From<DatabaseConfig> for PgConnectOptions {
    fn from(cfg: DatabaseConfig) -> Self {
        Self::new()
            .host(&cfg.host)
            .port(cfg.port)
            .username(&cfg.username)
            .password(&cfg.password)
            .database(&cfg.database)
    }
}

/// Create connection pool for PostgreSQL database.
fn connect_database_with(config: DatabaseConfig) -> PgPool {
    PgPool::connect_lazy_with(config.into())
}

/// Handler for health-check, that uses `/health` route, and returns `200 OK` status code.
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

/// Handler for database-health-check.
async fn health_check_db(State(db): State<PgPool>) -> StatusCode {
    let connection_result = sqlx::query("SELECT 1").fetch_one(&db).await;
    match connection_result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let conn_database_cfg = DatabaseConfig {
        host: "localhost".into(),
        port: 5432,
        username: "app".into(),
        password: "passwd".into(),
        database: "app".into(),
    };
    let conn_pool = connect_database_with(conn_database_cfg);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/health/db", get(health_check_db))
        .with_state(conn_pool); // register pool to state, due to share it with other handlers

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);

    Ok(axum::serve(listener, app).await?)
}
