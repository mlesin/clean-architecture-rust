use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CatFactPresenter {
    pub fact: String,
    pub id: i32,
}
