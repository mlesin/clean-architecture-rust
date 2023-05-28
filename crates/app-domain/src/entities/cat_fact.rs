#[derive(Debug, Clone)]
pub struct CatFactEntity {
    pub fact_txt: String,
    pub fact_id: i32,
}

impl CatFactEntity {
    pub fn new(fact_txt: String, fact_id: i32) -> Self {
        CatFactEntity { fact_txt, fact_id }
    }
}
