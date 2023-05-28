use async_trait::async_trait;
use std::error::Error;

use crate::connection::HttpConnection;
use app_core::services::AuthService;

pub struct CatFactsserviceHTTP {
    pub http_connection: HttpConnection,
    pub source: String,
}

#[async_trait(?Send)]
impl AuthService for CatFactsserviceHTTP {
    async fn login(&self, _username: &str, _password: &str) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
