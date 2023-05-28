use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DogFactJson {
    pub id: i32,
    pub fact: String,
}
