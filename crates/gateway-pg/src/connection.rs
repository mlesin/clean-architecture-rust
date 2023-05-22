use regex::{Captures, Regex};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::error::Error;

type DbPool = Pool<Postgres>;

pub struct DbConnection {
    pub db_name: String,
}

impl DbConnection {
    pub async fn get_pool(&self) -> Result<DbPool, Box<dyn Error>> {
        let re = Regex::new(r#"(^postgresql:\/\/[^@]+@[^\/]+)\/[^\/]+"#).unwrap();

        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let database = re.replace_all(&database_url, |caps: &Captures| format!("{}/{}", &caps[1], self.db_name));
        // (^postgresql:\/\/[^@]+@[^\/]+)\/[^\/]+
        // let database = format!("{}/{}", database_url, &self.db_name);

        Ok(PgPoolOptions::new().max_connections((num_cpus::get_physical() * 4) as u32).connect(&database).await?)
    }
}
