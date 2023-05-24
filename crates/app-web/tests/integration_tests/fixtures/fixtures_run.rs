use gateway_pg::connection::DbConnection;
use sqlx::{Postgres, QueryBuilder};

use crate::{
    integration_tests::fixtures::fixtures_struct::DogFactJson, utils::utils_file::read_from_file,
};

pub async fn execute_imports(conn: &DbConnection) {
    import_dog_facts_fixtures(conn).await;
}

async fn import_dog_facts_fixtures(conn: &DbConnection) {
    let json =
        read_from_file::<Vec<DogFactJson>>("tests/integration_tests/fixtures/dog_facts.json")
            .unwrap();

    let conn = conn
        .get_pool()
        .await
        .expect("couldn't get db connection from pool");
    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("INSERT INTO dog_facts(id, fact) ");
    const BIND_LIMIT: usize = 65535;
    query_builder.push_values(json.into_iter().take(BIND_LIMIT / 4), |mut b, dog| {
        b.push_bind(dog.id).push_bind(dog.fact);
    });

    let query = query_builder.build();
    query.execute(&conn).await.expect("can't insert data");
}
