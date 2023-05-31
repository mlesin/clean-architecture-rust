use async_trait::async_trait;
use regex::Regex;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Transaction};

use crate::{
    mappers::{CatFactDbMapper, DogFactDbMapper},
    models::{CatFact, DogFact},
};
use app_core::{
    mappers::service::ServiceMapper,
    services::{self, DBCatRepo, DBDogRepo, Persistence},
};
use app_domain::entities::{CatFactEntity, DogFactEntity};

#[derive(Clone)]
pub struct PersistencePG {
    pool: Pool<Postgres>,
}

impl PersistencePG {
    pub async fn new(db_name: &str) -> Result<Self, services::Error> {
        let re = Regex::new(r#"(^postgresql:\/\/[^@]+@[^\/]+)\/[^\/]+"#).unwrap();

        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let database = re.replace_all(&database_url, |caps: &regex::Captures| {
            format!("{}/{}", &caps[1], db_name)
        });
        // (^postgresql:\/\/[^@]+@[^\/]+)\/[^\/]+
        // let database = format!("{}/{}", database_url, &self.db_name);

        Ok(PersistencePG {
            pool: PgPoolOptions::new()
                .max_connections((num_cpus::get_physical() * 4) as u32)
                .connect(&database)
                .await
                .map_err(|_| services::Error::DatabaseError)?,
        })
    }
}

#[async_trait]
impl Persistence for PersistencePG {
    type Transaction = TransactionPG;
    async fn get_transaction(&self) -> Result<TransactionPG, services::Error> {
        let tx = self
            .pool
            .begin()
            .await
            .map_err(|_| services::Error::DatabaseError)?;

        Ok(TransactionPG(tx))
    }
}

pub struct TransactionPG(pub Transaction<'static, Postgres>);

#[async_trait()]
impl services::Transaction for TransactionPG {
    async fn commit(self: Self) -> Result<(), services::Error> {
        self.0
            .commit()
            .await
            .map_err(|_| services::Error::DatabaseError)
    }
    async fn rollback(self: Self) -> Result<(), services::Error> {
        self.0
            .rollback()
            .await
            .map_err(|_| services::Error::DatabaseError)
    }
}

#[derive(Clone, Copy)]
pub struct DogRepo {}

#[async_trait()]
impl DBDogRepo<PersistencePG> for DogRepo {
    async fn get_dog_fact_by_id(
        tx: &mut TransactionPG,
        dog_fact_id: i32,
    ) -> Result<DogFactEntity, services::Error> {
        let model = sqlx::query_as!(
            DogFact,
            "SELECT * FROM dog_facts WHERE id = $1",
            dog_fact_id
        )
        .fetch_one(&mut *tx.0)
        .await
        .map_err(|_| services::Error::DatabaseError)?;

        Ok(DogFactDbMapper::to_entity(model))
    }

    async fn get_all_dog_facts(
        tx: &mut TransactionPG,
    ) -> Result<Vec<DogFactEntity>, services::Error> {
        let models = sqlx::query_as!(DogFact, "SELECT * FROM dog_facts")
            .fetch_all(&mut *tx.0)
            .await
            .map_err(|_| services::Error::DatabaseError)?;

        Ok(models
            .into_iter()
            .map(DogFactDbMapper::to_entity)
            .collect::<Vec<DogFactEntity>>())
    }
}

#[derive(Clone, Copy)]
pub struct CatRepo {}

#[async_trait()]
impl DBCatRepo<PersistencePG> for CatRepo {
    async fn get_random_cat_fact(tx: &mut TransactionPG) -> Result<CatFactEntity, services::Error> {
        let model = sqlx::query_as!(CatFact, "SELECT * FROM cat_facts WHERE id = $1", 1)
            .fetch_one(&mut *tx.0)
            .await
            .map_err(|_| services::Error::DatabaseError)?;

        Ok(CatFactDbMapper::to_entity(model))
    }

    async fn get_all_cat_facts(
        tx: &mut TransactionPG,
    ) -> Result<Vec<CatFactEntity>, services::Error> {
        let models = sqlx::query_as!(CatFact, "SELECT * FROM cat_facts")
            .fetch_all(&mut *tx.0)
            .await
            .map_err(|_| services::Error::DatabaseError)?;

        Ok(models
            .into_iter()
            .map(CatFactDbMapper::to_entity)
            .collect::<Vec<CatFactEntity>>())
    }
}
