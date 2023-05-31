use async_trait::async_trait;
use thiserror::Error;

mod cat_repo;
mod dog_repo;

pub use cat_repo::*;
pub use dog_repo::*;

#[cfg(test)]
use mockall::{predicate::*, *};

/// An interface of any persistence
///
/// Persistence is anything that a Repository implementation could
/// use to store data.

#[cfg_attr(test, automock(type Transaction=MockTransaction;))]
#[async_trait]
pub trait Persistence: 'static + Send + Sync {
    type Transaction;
    /// Get a connection to persistence as
    async fn get_transaction(&self) -> Result<Self::Transaction, RepositoryError>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Transaction: Send + Sync {
    async fn commit(self: Self) -> Result<(), RepositoryError>;
    async fn rollback(self: Self) -> Result<(), RepositoryError>;
}

#[derive(Error, Debug)]
#[error("Repository error: {0}")]
pub struct RepositoryError(pub String);
