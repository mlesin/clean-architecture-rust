use async_trait::async_trait;

use app_domain::entities::{CatFactEntity, DogFactEntity};
use thiserror::Error;

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
    async fn get_transaction(&self) -> Result<Self::Transaction, Error>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Transaction: Send + Sync {
    async fn commit(self: Self) -> Result<(), Error>;
    async fn rollback(self: Self) -> Result<(), Error>;
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("wrong type")]
    WrongType,
    #[error("database error")]
    DatabaseError,
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait DBDogRepo<P: Persistence>: 'static
where
    <P as Persistence>::Transaction: Transaction,
{
    async fn get_all_dog_facts(tx: &mut P::Transaction) -> Result<Vec<DogFactEntity>, Error>;
    async fn get_dog_fact_by_id(
        tx: &mut P::Transaction,
        fact_id: i32,
    ) -> Result<DogFactEntity, Error>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait DBCatRepo<P: Persistence>: 'static
where
    <P as Persistence>::Transaction: Transaction,
{
    async fn get_all_cat_facts(tx: &mut P::Transaction) -> Result<Vec<CatFactEntity>, Error>;
    async fn get_random_cat_fact(tx: &mut P::Transaction) -> Result<CatFactEntity, Error>;
}
