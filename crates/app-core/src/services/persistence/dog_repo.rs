use app_domain::entities::DogFactEntity;
use async_trait::async_trait;

use super::{Persistence, RepositoryError, Transaction};

#[cfg(test)]
use mockall::{predicate::*, *};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait DogRepo<P: Persistence>: 'static
where
    <P as Persistence>::Transaction: Transaction,
{
    async fn get_all_dog_facts(
        tx: &mut P::Transaction,
    ) -> Result<Vec<DogFactEntity>, RepositoryError>;
    async fn get_dog_fact_by_id(
        tx: &mut P::Transaction,
        fact_id: i32,
    ) -> Result<Option<DogFactEntity>, RepositoryError>;
}
