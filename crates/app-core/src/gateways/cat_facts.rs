use async_trait::async_trait;

use app_domain::cat_fact_entity::CatFactEntity;

#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait CatFactsGateway {
    async fn get_random_cat_fact(&self) -> Result<CatFactEntity, Box<dyn Error>>;
    async fn get_all_cat_facts(&self) -> Result<Vec<CatFactEntity>, Box<dyn Error>>;
}
