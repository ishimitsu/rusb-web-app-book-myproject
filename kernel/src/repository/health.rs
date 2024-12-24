use async_trait::async_trait;

#[async_trait]
pub trait HealthCheckRepository: Send + Sync {
    /// try to connect to the database, and return true if success
    async fn check_db(&self) -> bool;
}
