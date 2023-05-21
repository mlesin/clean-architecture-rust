use crate::schema::*;

#[derive(Queryable, QueryableByName)]
#[diesel(table_name = dog_facts)]
pub struct DogFact {
    pub id: i32,
    pub fact: String,
}
