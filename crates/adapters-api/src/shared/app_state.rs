use application::repositories::{cat_facts::CatFactsRepository, dog_facts::DogFactsRepository};

pub struct AppState {
    pub app_name: String,
    pub cats_repository: Box<dyn CatFactsRepository + Send + Sync>,
    pub dogs_repository: Box<dyn DogFactsRepository + Send + Sync>,
}
