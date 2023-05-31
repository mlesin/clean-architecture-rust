use app_domain::entities::CatFactEntity;
use async_trait::async_trait;

use super::{Persistence, RepositoryError, Transaction};

#[cfg(test)]
use mockall::{predicate::*, *};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CatRepo<P: Persistence>: 'static
where
    <P as Persistence>::Transaction: Transaction,
{
    async fn get_all_cat_facts(
        tx: &mut P::Transaction,
    ) -> Result<Vec<CatFactEntity>, RepositoryError>;
    async fn get_random_cat_fact(tx: &mut P::Transaction)
        -> Result<CatFactEntity, RepositoryError>;
}
