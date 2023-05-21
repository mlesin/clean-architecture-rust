use diesel::Insertable;
use gateway_pg::schema::*;
use serde::Deserialize;

#[derive(Deserialize, Insertable, Debug)]
#[diesel(table_name = dog_facts)]
pub struct DogFactJson {
    pub id: i32,
    pub fact: String,
}
