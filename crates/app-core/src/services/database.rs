use async_trait::async_trait;

use app_domain::entities::{CatFactEntity, DogFactEntity};
use std::error::Error;

#[cfg(test)]
use mockall::{predicate::*, *};

// pub type AsyncFn<'a, Arg, Res, Err> =
//     dyn Fn(Arg) -> Pin<Box<dyn Future<Output = Result<Res, Err>> + Send + 'a>> + Sync + 'a;

#[cfg_attr(test, automock)]
#[async_trait()]
pub trait DatabaseService {
    async fn get_repo(&self) -> Result<Box<dyn DatabaseServiceRepo + Send + Sync>, Box<dyn Error>>;
}

#[cfg_attr(test, automock)]
#[async_trait()]
pub trait DatabaseServiceRepo {
    async fn commit(&mut self) -> Result<(), Box<dyn Error>>;
    async fn get_dog_fact_by_id(&self, fact_id: i32) -> Result<DogFactEntity, Box<dyn Error>>;
    async fn get_all_dog_facts(&self) -> Result<Vec<DogFactEntity>, Box<dyn Error>>;
    async fn get_random_cat_fact(&self) -> Result<CatFactEntity, Box<dyn Error>>;
    async fn get_all_cat_facts(&self) -> Result<Vec<CatFactEntity>, Box<dyn Error>>;
}
