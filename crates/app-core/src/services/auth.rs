use async_trait::async_trait;

#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;
#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait AuthService {
    async fn login(&self, username: &str, password: &str) -> Result<(), Box<dyn Error>>;
}
