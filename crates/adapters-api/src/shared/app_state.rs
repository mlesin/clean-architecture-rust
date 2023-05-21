use adapters_spi_db::db_dog_facts_repository::DogFactsRepositoryDB;
use adapters_spi_http::http_cat_facts_repository::CatFactsRepositoryHTTP;

pub struct AppState {
    pub app_name: String,
    pub cats_repository: CatFactsRepositoryHTTP,
    pub dogs_repository: DogFactsRepositoryDB,
}
