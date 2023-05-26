use async_trait::async_trait;
use regex::Regex;
//use sqlx::{error::BoxDynError, PgConnection};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Transaction};
use std::{error::Error, future::Future, pin::Pin, sync::Arc};
use tokio::sync::Mutex;

use crate::{mappers::DogFactDbMapper, models::DogFact};
use business::{
    gateways::dog_facts::{AsyncFn, DogFactsGateway, DogFactsGatewayRepo},
    mappers::gateway::GatewayMapper,
};
use entities::dog_fact_entity::DogFactEntity;

pub struct DogFactsGatewayRepoPG<'a> {
    transaction: Arc<Mutex<Transaction<'a, Postgres>>>,
}

impl<'a> DogFactsGatewayRepoPG<'a> {
    // pub async fn unit_of_work(
    //     pool: Pool<Postgres>,
    //     f: &AsyncFn<'_, Box<dyn DogFactsGatewayRepo + Send + Sync>, (), Box<dyn Error>>,
    // ) -> Result<(), Box<dyn Error>> {
    //     let tx = Arc::new(Mutex::new(pool.begin().await?));
    //     {
    //         let gw = Box::new(DogFactsGatewayRepoPG {
    //             transaction: tx.clone(),
    //         });
    //         f(gw).await?;
    //     }
    //     Arc::try_unwrap(tx)
    //         .expect("Can't unwrap Arc")
    //         .into_inner()
    //         .commit()
    //         .await?;
    //     Ok(())
    // }

    pub async fn new(pool: Pool<Postgres>) -> Result<DogFactsGatewayRepoPG<'a>, Box<dyn Error>> {
        Ok(DogFactsGatewayRepoPG {
            transaction: Arc::new(Mutex::new(pool.begin().await?)),
        })
    }
}

pub struct DogFactsGatewayPG {
    pool: Pool<Postgres>,
}

impl DogFactsGatewayPG {
    pub async fn get_pool(db_name: &str) -> Result<DogFactsGatewayPG, Box<dyn Error>> {
        let re = Regex::new(r#"(^postgresql:\/\/[^@]+@[^\/]+)\/[^\/]+"#).unwrap();

        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let database = re.replace_all(&database_url, |caps: &regex::Captures| {
            format!("{}/{}", &caps[1], db_name)
        });
        // (^postgresql:\/\/[^@]+@[^\/]+)\/[^\/]+
        // let database = format!("{}/{}", database_url, &self.db_name);

        Ok(DogFactsGatewayPG {
            pool: PgPoolOptions::new()
                .max_connections((num_cpus::get_physical() * 4) as u32)
                .connect(&database)
                .await?,
        })
    }
}

#[async_trait()]
impl DogFactsGateway for DogFactsGatewayPG {
    // async fn access_repo<'a>(
    //     &self,
    //     f: &AsyncFn<'a, Box<dyn DogFactsGatewayRepo + Send + Sync>, (), Box<dyn Error>>,
    // ) -> Result<(), Box<dyn Error>> {
    //     DogFactsGatewayRepoPG::unit_of_work(self.pool.clone(), f).await
    // }
    async fn get_repo(&self) -> Result<Box<dyn DogFactsGatewayRepo + Send + Sync>, Box<dyn Error>> {
        Ok(Box::new(
            DogFactsGatewayRepoPG::new(self.pool.clone()).await?,
        ))
    }
}

// async fn qqq(conn: &mut PgConnection) -> Result<(), BoxDynError> {
//     sqlx::query!("SELECT 1 as v").fetch_one(&mut *conn).await?;
//     sqlx::query!("SELECT 2 as v").fetch_one(&mut *conn).await?;

//     Ok(())
// }

#[async_trait()]
impl<'a> DogFactsGatewayRepo for DogFactsGatewayRepoPG<'a> {
    async fn commit(&mut self) -> Result<(), Box<dyn Error>> {
        Arc::try_unwrap(self.transaction)
            .expect("Can't unwrap Arc")
            .into_inner()
            .commit()
            .await?;
        Ok(())
    }

    async fn get_dog_fact_by_id(&self, dog_fact_id: i32) -> Result<DogFactEntity, Box<dyn Error>> {
        let mut conn = self.transaction.lock().await;

        let result = sqlx::query_as!(
            DogFact,
            "SELECT * FROM dog_facts WHERE id = $1",
            dog_fact_id
        )
        .fetch_one(&mut *conn)
        .await;

        // qqq(&mut *conn).await.unwrap();

        match result {
            Ok(model) => Ok(DogFactDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_all_dog_facts(&self) -> Result<Vec<DogFactEntity>, Box<dyn Error>> {
        let mut conn = self.transaction.lock().await;

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
    }
}
