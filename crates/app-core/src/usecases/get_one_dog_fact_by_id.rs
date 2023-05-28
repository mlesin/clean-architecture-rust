use async_trait::async_trait;

use crate::{
    gateways::dog_facts::DogFactsGateway, usecases::interfaces::UseCase,
    utils::error_handling_utils::ErrorHandlingUtils,
};
use app_domain::{dog_fact_entity::DogFactEntity, error::ApiError};

pub struct GetOneDogFactByIdUseCase<'a> {
    dog_fact_id: &'a i32,
    gateway: &'a dyn DogFactsGateway,
}

impl<'a> GetOneDogFactByIdUseCase<'a> {
    pub fn new(dog_fact_id: &'a i32, gateway: &'a dyn DogFactsGateway) -> Self {
        GetOneDogFactByIdUseCase {
            dog_fact_id,
            gateway,
        }
    }
}

#[async_trait(?Send)]
impl<'a> UseCase<DogFactEntity> for GetOneDogFactByIdUseCase<'a> {
    async fn execute(&self) -> Result<DogFactEntity, ApiError> {
        let dog_fact = {
            let mut repo = self.gateway.get_repo().await.unwrap(); //FIXME
            let fact = repo.get_dog_fact_by_id(*self.dog_fact_id).await;
            // transaction is dropped if repo gets out of scope without commit
            repo.commit().await.unwrap(); //FIXME
            fact
        };

        match dog_fact {
            Ok(dog_fact) => Ok(dog_fact),
            Err(e) => Err(ErrorHandlingUtils::business_error(
                "Cannot get single dog fact",
                Some(e),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;
    use std::io::{Error, ErrorKind};

    use crate::gateways::dog_facts::{MockDogFactsGateway, MockDogFactsGatewayRepo};
    use app_domain::dog_fact_entity::DogFactEntity;

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
                    .expect_get_dog_fact_by_id()
                    .with(eq(1))
                    .times(1)
                    .returning(|_| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));
                dog_fact_gateway_repo
                    .expect_commit()
                    .times(1)
                    .returning(|| Ok(()));
                Ok(Box::new(dog_fact_gateway_repo))
            });

        // when calling usecase
        let get_one_dog_fact_by_id_usecase = GetOneDogFactByIdUseCase::new(&1, &dog_fact_gateway);
        let data = get_one_dog_fact_by_id_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get single dog fact", result.message);
    }

    #[actix_rt::test]
    async fn test_should_return_one_result() {
        // given the "one dog fact by id" usecase repo returning one result
        let mut dog_fact_gateway = MockDogFactsGateway::new();
        dog_fact_gateway
            .expect_get_repo()
            .with()
            .times(1)
            .returning(|| {
                let mut dog_fact_gateway_repo = MockDogFactsGatewayRepo::new();
                dog_fact_gateway_repo
                    .expect_get_dog_fact_by_id()
                    .with(eq(1))
                    .times(1)
                    .returning(|_| {
                        Ok(DogFactEntity {
                            fact_id: 1,
                            fact: String::from("fact1"),
                        })
                    });
                dog_fact_gateway_repo
                    .expect_commit()
                    .times(1)
                    .returning(|| Ok(()));
                Ok(Box::new(dog_fact_gateway_repo))
            });

        // when calling usecase
        let get_one_dog_fact_by_id_usecase = GetOneDogFactByIdUseCase::new(&1, &dog_fact_gateway);
        let data = get_one_dog_fact_by_id_usecase.execute().await.unwrap();

        // then assert the result is the expected entity
        assert_eq!(data.fact_id, 1);
        assert_eq!(data.fact, "fact1");
    }
}
