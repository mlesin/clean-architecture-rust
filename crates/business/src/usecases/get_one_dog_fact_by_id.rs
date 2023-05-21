use async_trait::async_trait;

use crate::{gateways::dog_facts::DogFactsGateway, usecases::interfaces::UseCase, utils::error_handling_utils::ErrorHandlingUtils};
use entities::{dog_fact_entity::DogFactEntity, error::ApiError};

pub struct GetOneDogFactByIdUseCase<'a> {
    dog_fact_id: &'a i32,
    gateway: &'a dyn DogFactsGateway,
}

impl<'a> GetOneDogFactByIdUseCase<'a> {
    pub fn new(dog_fact_id: &'a i32, gateway: &'a dyn DogFactsGateway) -> Self {
        GetOneDogFactByIdUseCase { dog_fact_id, gateway }
    }
}

#[async_trait(?Send)]
impl<'a> UseCase<DogFactEntity> for GetOneDogFactByIdUseCase<'a> {
    async fn execute(&self) -> Result<DogFactEntity, ApiError> {
        let dog_fact = self.gateway.get_dog_fact_by_id(*self.dog_fact_id).await;

        match dog_fact {
            Ok(dog_fact) => Ok(dog_fact),
            Err(e) => Err(ErrorHandlingUtils::business_error("Cannot get single dog fact", Some(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;
    use std::io::{Error, ErrorKind};

    use crate::gateways::dog_facts::MockDogFactsGateway;
    use entities::dog_fact_entity::DogFactEntity;

    #[actix_rt::test]
    async fn test_should_return_error_with_generic_message_when_unexpected_repo_error() {
        // given the "all dog facts" usecase repo with an unexpected random error
        let mut dog_fact_gateway = MockDogFactsGateway::new();
        dog_fact_gateway
            .expect_get_dog_fact_by_id()
            .with(eq(1))
            .times(1)
            .returning(|_| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

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
        dog_fact_gateway.expect_get_dog_fact_by_id().with(eq(1)).times(1).returning(|_| {
            Ok(DogFactEntity {
                fact_id: 1,
                fact: String::from("fact1"),
            })
        });

        // when calling usecase
        let get_one_dog_fact_by_id_usecase = GetOneDogFactByIdUseCase::new(&1, &dog_fact_gateway);
        let data = get_one_dog_fact_by_id_usecase.execute().await.unwrap();

        // then assert the result is the expected entity
        assert_eq!(data.fact_id, 1);
        assert_eq!(data.fact, "fact1");
    }
}
