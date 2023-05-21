use async_trait::async_trait;

use crate::{gateways::cat_facts::CatFactsGateway, usecases::interfaces::UseCase, utils::error_handling_utils::ErrorHandlingUtils};
use entities::{cat_fact_entity::CatFactEntity, error::ApiError};

pub struct GetAllCatFactsUseCase<'a> {
    gateway: &'a dyn CatFactsGateway,
}

impl<'a> GetAllCatFactsUseCase<'a> {
    pub fn new(gateway: &'a dyn CatFactsGateway) -> Self {
        GetAllCatFactsUseCase { gateway }
    }
}

#[async_trait(?Send)]
impl<'a> UseCase<Vec<CatFactEntity>> for GetAllCatFactsUseCase<'a> {
    async fn execute(&self) -> Result<Vec<CatFactEntity>, ApiError> {
        let cat_facts = self.gateway.get_all_cat_facts().await;

        match cat_facts {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::business_error("Cannot get all cat facts", Some(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    use crate::gateways::cat_facts::CatFactsGateway;

    #[actix_rt::test]
    async fn test_should_return_generic_message_when_unexpected_repo_error() {
        // given the "all cat facts" usecase repo with an unexpected error
        let mut cat_fact_gateway = CatFactsGateway::new();
        cat_fact_gateway
            .expect_get_all_cat_facts()
            .with()
            .times(1)
            .returning(|| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

        // when calling usecase
        let get_all_cat_facts_usecase = GetAllCatFactsUseCase::new(&cat_fact_gateway);
        let data = get_all_cat_facts_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get all cat facts", result.message);
    }

    #[actix_rt::test]
    async fn test_should_return_empty_list() {
        // given the "all cat facts" usecase repo returning an empty list
        let mut cat_fact_gateway = CatFactsGateway::new();
        cat_fact_gateway.expect_get_all_cat_facts().with().times(1).returning(|| Ok(Vec::<CatFactEntity>::new()));

        // when calling usecase
        let get_all_cat_facts_usecase = GetAllCatFactsUseCase::new(&cat_fact_gateway);
        let data = get_all_cat_facts_usecase.execute().await.unwrap();

        // then assert the result is an empty list
        assert_eq!(data.len(), 0);
    }

    #[actix_rt::test]
    async fn test_should_return_list() {
        // given the "all cat facts" usecase repo returning a list of 2 entities
        let mut cat_fact_gateway = CatFactsGateway::new();
        cat_fact_gateway.expect_get_all_cat_facts().with().times(1).returning(|| {
            Ok(vec![
                CatFactEntity {
                    fact_txt: String::from("fact1"),
                    fact_length: 1,
                },
                CatFactEntity {
                    fact_txt: String::from("fact2"),
                    fact_length: 2,
                },
            ])
        });

        // when calling usecase
        let get_all_cat_facts_usecase = GetAllCatFactsUseCase::new(&cat_fact_gateway);
        let data = get_all_cat_facts_usecase.execute().await.unwrap();

        // then assert the result is an empty list
        assert_eq!(data.len(), 2);
    }
}
