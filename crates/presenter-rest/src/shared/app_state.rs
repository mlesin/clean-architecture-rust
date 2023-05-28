use app_core::services::{AuthService, DatabaseService};

pub struct AppState {
    pub app_name: String,
    pub auth_service: Box<dyn AuthService + Send + Sync>,
    pub db_service: Box<dyn DatabaseService + Send + Sync>,
}
