use std::marker::PhantomData;

use app_core::services::AuthService;

pub struct AppState<P, DR, CR> {
    pub app_name: String,
    pub auth_service: Box<dyn AuthService + Send + Sync>,
    pub persistence_service: P,
    pub dog_repo: PhantomData<DR>,
    pub cat_repo: PhantomData<CR>,
}
