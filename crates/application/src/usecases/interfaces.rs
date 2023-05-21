use async_trait::async_trait;
use domain::error::ApiError;

#[async_trait(?Send)]
pub trait UseCase<T> {
    async fn execute(&self) -> Result<T, ApiError>;
}
