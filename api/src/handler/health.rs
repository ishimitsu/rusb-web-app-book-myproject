use axum::{extract::State, http::StatusCode};
use registry::AppRegistry;

/// Handler for health-check, that uses `/health` route, and returns `200 OK` status code.
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

/// Handler for database-health-check.
pub async fn health_check_db(State(registry): State<AppRegistry>) -> StatusCode {
    if registry.health_check_repository().check_db().await {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
