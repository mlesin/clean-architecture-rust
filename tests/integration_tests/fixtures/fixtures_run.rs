use adapters_spi_db::db::db_connection::DbConnection;
use adapters_spi_db::db::schema::dog_facts::dsl::*;
use diesel::{insert_into, RunQueryDsl};

use crate::{integration_tests::fixtures::fixtures_struct::DogFactJson, utils::utils_file::read_from_file};

pub fn execute_imports(conn: &DbConnection) {
    import_dog_facts_fixtures(conn);
}

fn import_dog_facts_fixtures(conn: &DbConnection) {
    let json = read_from_file::<Vec<DogFactJson>>("tests/integration_tests/fixtures/dog_facts.json").unwrap();

    let mut conn = conn.get_pool().get().expect("couldn't get db connection from pool");
    insert_into(dog_facts).values(&json).execute(&mut conn).unwrap();
}
