use crate::integration_tests::fixtures::fixtures_run;
use fixtures_run::execute_imports;
use gateway_pg::connection::DbConnection;

pub struct TestContextPostgreSQL {
    pub base_url: String,
    pub db_name: String,
}

impl TestContextPostgreSQL {
    pub async fn new(base_url: &str, db_name: &str) -> Self {
        let db_connection_postgres_db = DbConnection { db_name: db_name.to_string() };
        execute_imports(&db_connection_postgres_db).await;

        Self {
            base_url: base_url.to_string(),
            db_name: db_name.to_string(),
        }
    }
}
