use async_trait::async_trait;

use app_domain::dog_fact_entity::DogFactEntity;
use std::{error::Error, future::Future, pin::Pin};

#[cfg(test)]
use mockall::{predicate::*, *};

pub type AsyncFn<'a, Arg, Res, Err> =
    dyn Fn(Arg) -> Pin<Box<dyn Future<Output = Result<Res, Err>> + Send + 'a>> + Sync + 'a;

#[cfg_attr(test, automock)]
#[async_trait()]
pub trait DogFactsGateway {
    async fn get_repo(&self) -> Result<Box<dyn DogFactsGatewayRepo + Send + Sync>, Box<dyn Error>>;
}

#[cfg_attr(test, automock)]
#[async_trait()]
pub trait DogFactsGatewayRepo {
    async fn commit(&mut self) -> Result<(), Box<dyn Error>>;
    async fn get_dog_fact_by_id(&self, fact_id: i32) -> Result<DogFactEntity, Box<dyn Error>>;
    async fn get_all_dog_facts(&self) -> Result<Vec<DogFactEntity>, Box<dyn Error>>;
}
