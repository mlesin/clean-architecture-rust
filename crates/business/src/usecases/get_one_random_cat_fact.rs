use async_trait::async_trait;

use crate::{gateways::cat_facts::CatFactsGateway, usecases::interfaces::UseCase, utils::error_handling_utils::ErrorHandlingUtils};
use entities::{cat_fact_entity::CatFactEntity, error::ApiError};

pub struct GetOneRandomCatFactUseCase<'a> {
    gateway: &'a dyn CatFactsGateway,
}

impl<'a> GetOneRandomCatFactUseCase<'a> {
    pub fn new(gateway: &'a dyn CatFactsGateway) -> Self {
        GetOneRandomCatFactUseCase { gateway }
    }
}

#[async_trait(?Send)]
impl<'a> UseCase<CatFactEntity> for GetOneRandomCatFactUseCase<'a> {
    async fn execute(&self) -> Result<CatFactEntity, ApiError> {
        let cat_fact = self.gateway.get_random_cat_fact().await;

        match cat_fact {
            Ok(fact) => Ok(fact),
            Err(e) => Err(ErrorHandlingUtils::business_error("Cannot get random cat fact", Some(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    use crate::{gateways::cat_facts::MockCatFactsGateway, usecases::get_one_random_cat_fact::GetOneRandomCatFactUseCase};

    #[actix_rt::test]
    async fn test_should_return_generic_message_when_unexpected_repo_error() {
        // given the "all cat facts" usecase repo with an unexpected error
        let mut cat_fact_gateway = MockCatFactsGateway::new();
        cat_fact_gateway
            .expect_get_random_cat_fact()
            .with()
            .times(1)
            .returning(|| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

        // when calling usecase
        let get_one_random_cat_fact_usecase = GetOneRandomCatFactUseCase::new(&cat_fact_gateway);
        let data = get_one_random_cat_fact_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get random cat fact", result.message);
    }

    #[actix_rt::test]
    async fn test_should_return_one_result() {
        // given the "one random cat fact" usecase repo returning one result
        let mut cat_fact_gateway = MockCatFactsGateway::new();
        cat_fact_gateway.expect_get_random_cat_fact().with().times(1).returning(|| {
            Ok(CatFactEntity {
                fact_txt: String::from("fact1"),
                fact_length: 1,
            })
        });

        // when calling usecase
        let get_one_random_cat_fact_usecase = GetOneRandomCatFactUseCase::new(&cat_fact_gateway);
        let data = get_one_random_cat_fact_usecase.execute().await.unwrap();

        // then assert the result is the expected entity
        assert_eq!(data.fact_txt, "fact1");
        assert_eq!(data.fact_length, 1);
    }
}
