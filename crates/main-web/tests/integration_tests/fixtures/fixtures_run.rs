use sqlx::{PgConnection, Postgres, QueryBuilder};

use crate::{
    integration_tests::fixtures::fixtures_struct::DogFactJson, utils::utils_file::read_from_file,
};

pub async fn execute_imports(conn: &mut PgConnection) {
    import_dog_facts_fixtures(conn).await;
    import_cat_facts_fixtures(conn).await;
}

async fn import_dog_facts_fixtures(conn: &mut PgConnection) {
    let json =
        read_from_file::<Vec<DogFactJson>>("tests/integration_tests/fixtures/dog_facts.json")
            .unwrap();

    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("INSERT INTO dog_facts(id, fact) ");
    const BIND_LIMIT: usize = 65535;
    query_builder.push_values(json.into_iter().take(BIND_LIMIT / 4), |mut b, dog| {
        b.push_bind(dog.id).push_bind(dog.fact);
    });

    let query = query_builder.build();
    query.execute(conn).await.expect("can't insert data");
}

async fn import_cat_facts_fixtures(conn: &mut PgConnection) {
    let json =
        read_from_file::<Vec<DogFactJson>>("tests/integration_tests/fixtures/cat_facts.json")
            .unwrap();

    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("INSERT INTO cat_facts(id, fact) ");
    const BIND_LIMIT: usize = 65535;
    query_builder.push_values(json.into_iter().take(BIND_LIMIT / 4), |mut b, cat| {
        b.push_bind(cat.id).push_bind(cat.fact);
    });

    let query = query_builder.build();
    query.execute(conn).await.expect("can't insert data");
}
