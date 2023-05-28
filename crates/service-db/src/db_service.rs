use async_trait::async_trait;
use regex::Regex;
//use sqlx::{error::BoxDynError, PgConnection};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Transaction};
use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;

use crate::{
    mappers::{CatFactDbMapper, DogFactDbMapper},
    models::{CatFact, DogFact},
};
use app_core::{
    mappers::service::ServiceMapper,
    services::{DatabaseService, DatabaseServiceRepo},
};
use app_domain::entities::{CatFactEntity, DogFactEntity};

struct DatabaseServiceRepoPG<'a> {
    transaction: Arc<Mutex<Option<Transaction<'a, Postgres>>>>,
}

impl<'a> DatabaseServiceRepoPG<'a> {
    pub async fn new(pool: Pool<Postgres>) -> Result<DatabaseServiceRepoPG<'a>, Box<dyn Error>> {
        Ok(DatabaseServiceRepoPG {
            transaction: Arc::new(Mutex::new(Some(pool.begin().await?))),
        })
    }
}

pub struct DatabaseServicePG {
    pool: Pool<Postgres>,
}

impl DatabaseServicePG {
    pub async fn new(db_name: &str) -> Result<DatabaseServicePG, Box<dyn Error>> {
        let re = Regex::new(r#"(^postgresql:\/\/[^@]+@[^\/]+)\/[^\/]+"#).unwrap();

        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let database = re.replace_all(&database_url, |caps: &regex::Captures| {
            format!("{}/{}", &caps[1], db_name)
        });
        // (^postgresql:\/\/[^@]+@[^\/]+)\/[^\/]+
        // let database = format!("{}/{}", database_url, &self.db_name);

        Ok(DatabaseServicePG {
            pool: PgPoolOptions::new()
                .max_connections((num_cpus::get_physical() * 4) as u32)
                .connect(&database)
                .await?,
        })
    }
}

#[async_trait()]
impl DatabaseService for DatabaseServicePG {
    async fn get_repo(&self) -> Result<Box<dyn DatabaseServiceRepo + Send + Sync>, Box<dyn Error>> {
        Ok(Box::new(
            DatabaseServiceRepoPG::new(self.pool.clone()).await?,
        ))
    }
}

#[async_trait()]
impl<'a> DatabaseServiceRepo for DatabaseServiceRepoPG<'a> {
    async fn commit(&mut self) -> Result<(), Box<dyn Error>> {
        let tx_opt = self.transaction.lock().await.take();
        if let Some(tx) = tx_opt {
            tx.commit().await?;
        }
        Ok(())
    }

    async fn get_dog_fact_by_id(&self, dog_fact_id: i32) -> Result<DogFactEntity, Box<dyn Error>> {
        if let Some(conn) = self.transaction.lock().await.as_mut() {
            let result = sqlx::query_as!(
                DogFact,
                "SELECT * FROM dog_facts WHERE id = $1",
                dog_fact_id
            )
            .fetch_one(&mut *conn)
            .await;

            match result {
                Ok(model) => Ok(DogFactDbMapper::to_entity(model)),
                Err(e) => Err(Box::new(e)),
            }
        } else {
            todo!()
        }
    }

    async fn get_all_dog_facts(&self) -> Result<Vec<DogFactEntity>, Box<dyn Error>> {
        if let Some(conn) = self.transaction.lock().await.as_mut() {
            let results = sqlx::query_as!(DogFact, "SELECT * FROM dog_facts")
                .fetch_all(&mut *conn)
                .await;

            match results {
                Ok(models) => Ok(models
                    .into_iter()
                    .map(DogFactDbMapper::to_entity)
                    .collect::<Vec<DogFactEntity>>()),
                Err(e) => Err(Box::new(e)),
            }
        } else {
            todo!()
        }
    }

    async fn get_random_cat_fact(&self) -> Result<CatFactEntity, Box<dyn Error>> {
        if let Some(conn) = self.transaction.lock().await.as_mut() {
            let result = sqlx::query_as!(CatFact, "SELECT * FROM cat_facts WHERE id = $1", 1)
                .fetch_one(&mut *conn)
                .await;

            match result {
                Ok(model) => Ok(CatFactDbMapper::to_entity(model)),
                Err(e) => Err(Box::new(e)),
            }
        } else {
            todo!()
        }
    }

    async fn get_all_cat_facts(&self) -> Result<Vec<CatFactEntity>, Box<dyn Error>> {
        if let Some(conn) = self.transaction.lock().await.as_mut() {
            let results = sqlx::query_as!(CatFact, "SELECT * FROM cat_facts")
                .fetch_all(&mut *conn)
                .await;

            match results {
                Ok(models) => Ok(models
                    .into_iter()
                    .map(CatFactDbMapper::to_entity)
                    .collect::<Vec<CatFactEntity>>()),
                Err(e) => Err(Box::new(e)),
            }
        } else {
            todo!()
        }
    }
}
