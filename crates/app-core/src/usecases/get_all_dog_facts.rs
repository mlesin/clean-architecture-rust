use async_trait::async_trait;

use crate::{
    gateways::dog_facts::DogFactsGateway, usecases::interfaces::UseCase,
    utils::error_handling_utils::ErrorHandlingUtils,
};
use app_domain::{entities::DogFactEntity, error::ApiError};

pub struct GetAllDogFactsUseCase<'a> {
    gateway: &'a dyn DogFactsGateway,
}

impl<'a> GetAllDogFactsUseCase<'a> {
    pub fn new(gateway: &'a dyn DogFactsGateway) -> Self {
        GetAllDogFactsUseCase { gateway }
    }
}

#[async_trait(?Send)]
impl<'a> UseCase<Vec<DogFactEntity>> for GetAllDogFactsUseCase<'a> {
    async fn execute(&self) -> Result<Vec<DogFactEntity>, ApiError> {
        let dog_facts = {
            let mut repo = self.gateway.get_repo().await.unwrap(); //FIXME
            let facts = repo.get_all_dog_facts().await;
            // transaction is dropped if repo gets out of scope without commit
            repo.commit().await.unwrap(); //FIXME
            facts
        };

        match dog_facts {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::business_error(
                "Cannot get all dog facts",
                Some(e),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    use crate::gateways::dog_facts::{MockDogFactsGateway, MockDogFactsGatewayRepo};
    use app_domain::entities::DogFactEntity;

    #[actix_rt::test]
    async fn test_should_return_error_with_generic_message_when_unexpected_repo_error() {
        // given the "all dog facts" usecase repo with an unexpected random error
        let mut dog_fact_gateway = MockDogFactsGateway::new();
        dog_fact_gateway
            .expect_get_repo()
            .with()
            .times(1)
            .returning(|| {
                let mut dog_fact_gateway_repo = MockDogFactsGatewayRepo::new();
                dog_fact_gateway_repo
                    .expect_get_all_dog_facts()
                    .with()
                    .times(1)
                    .returning(|| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));
                dog_fact_gateway_repo
                    .expect_commit()
                    .times(1)
                    .returning(|| Ok(()));
                Ok(Box::new(dog_fact_gateway_repo))
            });

        // when calling usecase
        let get_all_dog_facts_usecase = GetAllDogFactsUseCase::new(&dog_fact_gateway);
        let data = get_all_dog_facts_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get all dog facts", result.message);
    }

    #[actix_rt::test]
    async fn test_should_return_empty_list() {
        // given the "all dog facts" usecase repo returning an empty list
        let mut dog_fact_gateway = MockDogFactsGateway::new();
        dog_fact_gateway
            .expect_get_repo()
            .with()
            .times(1)
            .returning(|| {
                let mut dog_fact_gateway_repo = MockDogFactsGatewayRepo::new();
                dog_fact_gateway_repo
                    .expect_get_all_dog_facts()
                    .with()
                    .times(1)
                    .returning(|| Ok(Vec::<DogFactEntity>::new()));
                dog_fact_gateway_repo
                    .expect_commit()
                    .times(1)
                    .returning(|| Ok(()));
                Ok(Box::new(dog_fact_gateway_repo))
            });

        // when calling usecase
        let get_all_dog_facts_usecase = GetAllDogFactsUseCase::new(&dog_fact_gateway);
        let data = get_all_dog_facts_usecase.execute().await.unwrap();

        // then assert the result is an empty list
        assert_eq!(data.len(), 0);
    }

    #[actix_rt::test]
    async fn test_should_return_list() {
        // given the "all dog facts" usecase repo returning a list of 2 entities
        let mut dog_fact_gateway = MockDogFactsGateway::new();
        dog_fact_gateway
            .expect_get_repo()
            .with()
            .times(1)
            .returning(|| {
                let mut dog_fact_gateway_repo = MockDogFactsGatewayRepo::new();
                dog_fact_gateway_repo
                    .expect_get_all_dog_facts()
                    .with()
                    .times(1)
                    .returning(|| {
                        Ok(vec![
                            DogFactEntity {
                                fact_id: 1,
                                fact: String::from("fact1"),
                            },
                            DogFactEntity {
                                fact_id: 2,
                                fact: String::from("fact2"),
                            },
                        ])
                    });
                dog_fact_gateway_repo
                    .expect_commit()
                    .times(1)
                    .returning(|| Ok(()));
                Ok(Box::new(dog_fact_gateway_repo))
            });

        // when calling usecase
        let get_all_dog_facts_usecase = GetAllDogFactsUseCase::new(&dog_fact_gateway);
        let data = get_all_dog_facts_usecase.execute().await.unwrap();

        // then assert the result is an empty list
        assert_eq!(data.len(), 2);
    }
}
