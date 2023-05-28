use app_core::gateways::{cat_facts::CatFactsGateway, dog_facts::DogFactsGateway};

pub struct AppState {
    pub app_name: String,
    pub cats_gateway: Box<dyn CatFactsGateway + Send + Sync>,
    pub dogs_gateway: Box<dyn DogFactsGateway + Send + Sync>,
}
