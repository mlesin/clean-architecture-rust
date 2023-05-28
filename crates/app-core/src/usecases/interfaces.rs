use app_domain::error::AppError;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait UseCase<T> {
    async fn execute(&self) -> Result<T, AppError>;
}
