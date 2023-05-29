use async_trait::async_trait;
use regex::Regex;
//use sqlx::{error::BoxDynError, PgConnection};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;

use crate::{
    mappers::{CatFactDbMapper, DogFactDbMapper},
    models::{CatFact, DogFact},
};
use app_core::{
    mappers::service::ServiceMapper,
    services::{
        self, Caster, Connection, DatabaseService, DatabaseServiceRepo, Persistence, Transaction,
    },
};
use app_domain::entities::{CatFactEntity, DogFactEntity};

// pub struct PersistencePG {
//     pool: Pool<Postgres>,
// }

// #[async_trait]
// impl Persistence for PersistencePG {
//     async fn get_connection(&self) -> Result<Box<dyn Connection>, services::Error> {
//         let conn = *self
//             .pool
//             .acquire()
//             .await
//             .map_err(|_| services::Error::DatabaseError)?;
//         Ok(Box::new(ConnectionPG(conn)))
//     }
// }

pub struct ConnectionPG(pub sqlx::PgConnection);

impl<'a> dyno::Tag<'a> for ConnectionPG {
    type Type = ConnectionPG;
}
impl Connection for ConnectionPG {
    fn start_transaction<'a>(
        &'a mut self,
    ) -> Result<Box<dyn Transaction<'a> + 'a>, services::Error> {
        todo!()
        // Ok(Box::new(TransactionPG(self.0.transaction()?)))
    }

    fn cast<'b>(&'b mut self) -> Caster<'b, 'static> {
        Caster::new::<ConnectionPG>(self)
    }
}

// pub struct TransactionPG<'a>(pub sqlx::Transaction<'a, Postgres>);

// impl<'a> dyno::Tag<'a> for TransactionPG<'static> {
//     type Type = TransactionPG<'a>;
// }

// impl<'a> Transaction<'a> for TransactionPG<'a> {
//     fn commit(self: Box<Self>) -> Result<(), services::Error> {
//         Ok(((self.0) as sqlx::Transaction<'a>).commit()?)
//     }

//     fn rollback(self: Box<Self>) -> Result<(), services::Error> {
//         Ok(((self.0) as sqlx::Transaction<'a>).rollback()?)
//     }

//     fn cast<'caster>(&'caster mut self) -> Caster<'caster, 'a>
//     where
//         'a: 'caster,
//     {
//         Caster::new::<TransactionPG<'static>>(self)
//     }
// }

////////////////////////////

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

#[async_trait]
impl DatabaseService for DatabaseServicePG {
    async fn get_repo(
        &self,
    ) -> Result<Box<dyn DatabaseServiceRepo + Send + Sync>, services::Error> {
        Ok(Box::new(
            DatabaseServiceRepoPG::new(self.pool.clone()).await?,
        ))
    }
}

// struct DatabaseServiceConnPG<'a> {
//     transaction: Arc<Mutex<Option<Transaction<'a, Postgres>>>>,
// }

// #[async_trait]
// impl<'a> DBConn<'a> for DatabaseServiceConnPG<'a> {
//     async fn start_tx(&'a mut self) -> Result<Box<dyn DBConn + Send + Sync + 'a>, Box<dyn Error>> {
//         let q = &mut *self.transaction.lock().await;
//         if let Some(tx) = q {
//             Ok(Box::new(DatabaseServiceConnPG {
//                 transaction: Arc::new(Mutex::new(Some(tx.acquire().await?.begin().await?))),
//             }))
//         } else {
//             todo!()
//         }
//     }

//     async fn commit(&mut self) -> Result<(), Box<dyn Error>> {
//         todo!()
//     }
// }

// #[async_trait]
// impl DBSrv for DatabaseServicePG {
//     async fn get_conn<'a>(&self) -> Result<Box<dyn DBConn + Send + Sync + 'a>, Box<dyn Error>> {
//         Ok(Box::new(DatabaseServiceConnPG {
//             transaction: Arc::new(Mutex::new(Some(self.pool.begin().await?))),
//         }))
//     }
// }

struct DatabaseServiceRepoPG<'a> {
    transaction: Arc<Mutex<Option<sqlx::Transaction<'a, Postgres>>>>,
}

impl<'a> DatabaseServiceRepoPG<'a> {
    pub async fn new(pool: Pool<Postgres>) -> Result<DatabaseServiceRepoPG<'a>, services::Error> {
        Ok(DatabaseServiceRepoPG {
            transaction: Arc::new(Mutex::new(Some(
                pool.begin()
                    .await
                    .map_err(|_| services::Error::DatabaseError)?,
            ))),
        })
    }
}

#[async_trait()]
impl<'a> DatabaseServiceRepo for DatabaseServiceRepoPG<'a> {
    async fn commit(&mut self) -> Result<(), services::Error> {
        let tx_opt = self.transaction.lock().await.take();
        if let Some(tx) = tx_opt {
            tx.commit()
                .await
                .map_err(|_| services::Error::DatabaseError)?;
        }
        Ok(())
    }

    async fn get_dog_fact_by_id(&self, dog_fact_id: i32) -> Result<DogFactEntity, services::Error> {
        if let Some(conn) = self.transaction.lock().await.as_mut() {
            let model = sqlx::query_as!(
                DogFact,
                "SELECT * FROM dog_facts WHERE id = $1",
                dog_fact_id
            )
            .fetch_one(&mut *conn)
            .await
            .map_err(|_| services::Error::DatabaseError)?;

            Ok(DogFactDbMapper::to_entity(model))
        } else {
            Err(services::Error::DatabaseError)
        }
    }

    async fn get_all_dog_facts(&self) -> Result<Vec<DogFactEntity>, services::Error> {
        if let Some(conn) = self.transaction.lock().await.as_mut() {
            let models = sqlx::query_as!(DogFact, "SELECT * FROM dog_facts")
                .fetch_all(&mut *conn)
                .await
                .map_err(|_| services::Error::DatabaseError)?;

            Ok(models
                .into_iter()
                .map(DogFactDbMapper::to_entity)
                .collect::<Vec<DogFactEntity>>())
        } else {
            Err(services::Error::DatabaseError)
        }
    }

    async fn get_random_cat_fact(&self) -> Result<CatFactEntity, services::Error> {
        if let Some(conn) = self.transaction.lock().await.as_mut() {
            let model = sqlx::query_as!(CatFact, "SELECT * FROM cat_facts WHERE id = $1", 1)
                .fetch_one(&mut *conn)
                .await
                .map_err(|_| services::Error::DatabaseError)?;

            Ok(CatFactDbMapper::to_entity(model))
        } else {
            Err(services::Error::DatabaseError)
        }
    }

    async fn get_all_cat_facts(&self) -> Result<Vec<CatFactEntity>, services::Error> {
        if let Some(conn) = self.transaction.lock().await.as_mut() {
            let models = sqlx::query_as!(CatFact, "SELECT * FROM cat_facts")
                .fetch_all(&mut *conn)
                .await
                .map_err(|_| services::Error::DatabaseError)?;

            Ok(models
                .into_iter()
                .map(CatFactDbMapper::to_entity)
                .collect::<Vec<CatFactEntity>>())
        } else {
            todo!()
        }
    }
}
