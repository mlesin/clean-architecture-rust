// use business::gateways::dog_facts::DogFactsGatewayRepo;
// use regex::{Captures, Regex};
// use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
// use std::{error::Error, future::Future, pin::Pin, sync::Arc};
// use tokio::sync::Mutex;

// use crate::dog_facts_gateway::DogFactsGatewayRepoPG;

// pub struct DbConnection {
//     pool: Pool<Postgres>,
// }

// impl DbConnection {
// pub async fn get_pool(db_name: &str) -> Result<DbConnection, Box<dyn Error>> {
//     let re = Regex::new(r#"(^postgresql:\/\/[^@]+@[^\/]+)\/[^\/]+"#).unwrap();

//     let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     let database = re.replace_all(&database_url, |caps: &Captures| {
//         format!("{}/{}", &caps[1], db_name)
//     });
//     // (^postgresql:\/\/[^@]+@[^\/]+)\/[^\/]+
//     // let database = format!("{}/{}", database_url, &self.db_name);

//     Ok(DbConnection {
//         pool: PgPoolOptions::new()
//             .max_connections((num_cpus::get_physical() * 4) as u32)
//             .connect(&database)
//             .await?,
//     })
// }

// pub async fn unit_of_work<'a>(
//     &self,
//     f: &AsyncFn<'_, Box<dyn DogFactsGateway>, Result<(), Box<dyn Error>>>,
// ) -> Result<(), Box<dyn Error>> {
//     let tx = Arc::new(Mutex::new(self.pool.begin().await?));
//     {
//         let gw = Box::new(DogFactsgatewayPG {
//             transaction: tx.clone(),
//         });
//         f(gw).await??;
//     }
//     Arc::try_unwrap(tx)
//         .expect("Can't unwrap Arc")
//         .into_inner()
//         .commit()
//         .await?;
//     Ok(())
// }
// }
