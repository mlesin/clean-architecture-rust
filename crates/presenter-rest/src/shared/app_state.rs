use app_core::services::AuthService;

pub struct AppState<P> {
    pub auth_service: Box<dyn AuthService + Send + Sync>,
    pub persistence_service: P,
}
