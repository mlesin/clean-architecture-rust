use app_domain::error::ApiError;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait UseCase<T> {
    async fn execute(&self) -> Result<T, ApiError>;
}
