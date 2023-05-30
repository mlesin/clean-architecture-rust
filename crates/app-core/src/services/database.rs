use async_trait::async_trait;

use app_domain::entities::{CatFactEntity, DogFactEntity};
use thiserror::Error;

#[cfg(test)]
use mockall::{predicate::*, *};

/// An interface of any persistence
///
/// Persistence is anything that a Repository implementation could
/// use to store data.

// #[cfg_attr(test, automock)]
#[async_trait]
pub trait Persistence<'a>: Send + Sync {
    type Transaction;
    /// Get a connection to persistence
    async fn get_transaction(&self) -> Result<Self::Transaction, Error>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Transaction {
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

#[async_trait]
pub trait DBDogRepo<'p, P: Persistence<'p>>
where
    <P as Persistence<'p>>::Transaction: Transaction,
{
    async fn get_all_dog_facts(tx: &mut P::Transaction) -> Result<Vec<DogFactEntity>, Error>;
    async fn get_dog_fact_by_id(
        tx: &mut P::Transaction,
        fact_id: i32,
    ) -> Result<DogFactEntity, Error>;
}

#[async_trait]
pub trait DBCatRepo<'p, P: Persistence<'p>>
where
    <P as Persistence<'p>>::Transaction: Transaction,
{
    async fn get_all_cat_facts(tx: &mut P::Transaction) -> Result<Vec<CatFactEntity>, Error>;
    async fn get_random_cat_fact(tx: &mut P::Transaction) -> Result<CatFactEntity, Error>;
}
