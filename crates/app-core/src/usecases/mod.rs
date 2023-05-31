pub mod get_all_cat_facts;
pub mod get_all_dog_facts;
pub mod get_one_dog_fact_by_id;
pub mod get_one_random_cat_fact;

use thiserror::Error;

use crate::services::RepositoryError;

#[derive(Error, Debug)]
pub enum UseCaseError {
    #[error("Repository error: {0}")]
    Repository(String),
    #[error("Business error: {0}")]
    Business(String),
    #[error("Error: not authenticated or token expired")]
    Unauthorized(String),
    #[error("Error: resource not allowed")]
    Forbidden(String),
}

impl From<RepositoryError> for UseCaseError {
    fn from(value: RepositoryError) -> Self {
        Self::Repository(value.0)
    }
}
