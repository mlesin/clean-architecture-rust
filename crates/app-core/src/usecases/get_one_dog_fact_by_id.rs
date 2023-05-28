use async_trait::async_trait;

use crate::{
    services::DatabaseService, usecases::interfaces::UseCase,
    utils::error_handling_utils::ErrorHandlingUtils,
};
use app_domain::{entities::DogFactEntity, error::AppError};

pub struct GetOneDogFactByIdUseCase<'a> {
    dog_fact_id: &'a i32,
    service: &'a dyn DatabaseService,
}

impl<'a> GetOneDogFactByIdUseCase<'a> {
    pub fn new(dog_fact_id: &'a i32, service: &'a dyn DatabaseService) -> Self {
        GetOneDogFactByIdUseCase {
            dog_fact_id,
            service,
        }
    }
}

#[async_trait(?Send)]
impl<'a> UseCase<DogFactEntity> for GetOneDogFactByIdUseCase<'a> {
    async fn execute(&self) -> Result<DogFactEntity, AppError> {
        let dog_fact = {
            let mut repo = self.service.get_repo().await.unwrap(); //FIXME
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

    use crate::services::{MockDatabaseService, MockDatabaseServiceRepo};
    use app_domain::entities::DogFactEntity;

    #[actix_rt::test]
    async fn test_should_return_error_with_generic_message_when_unexpected_repo_error() {
        // given the "all dog facts" usecase repo with an unexpected random error
        let mut db_service = MockDatabaseService::new();
        db_service.expect_get_repo().with().times(1).returning(|| {
            let mut db_service_repo = MockDatabaseServiceRepo::new();
            db_service_repo
                .expect_get_dog_fact_by_id()
                .with(eq(1))
                .times(1)
                .returning(|_| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));
            db_service_repo
                .expect_commit()
                .times(1)
                .returning(|| Ok(()));
            Ok(Box::new(db_service_repo))
        });

        // when calling usecase
        let get_one_dog_fact_by_id_usecase = GetOneDogFactByIdUseCase::new(&1, &db_service);
        let data = get_one_dog_fact_by_id_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get single dog fact", result.message);
    }

    #[actix_rt::test]
    async fn test_should_return_one_result() {
        // given the "one dog fact by id" usecase repo returning one result
        let mut db_service = MockDatabaseService::new();
        db_service.expect_get_repo().with().times(1).returning(|| {
            let mut db_service_repo = MockDatabaseServiceRepo::new();
            db_service_repo
                .expect_get_dog_fact_by_id()
                .with(eq(1))
                .times(1)
                .returning(|_| {
                    Ok(DogFactEntity {
                        fact_id: 1,
                        fact: String::from("fact1"),
                    })
                });
            db_service_repo
                .expect_commit()
                .times(1)
                .returning(|| Ok(()));
            Ok(Box::new(db_service_repo))
        });

        // when calling usecase
        let get_one_dog_fact_by_id_usecase = GetOneDogFactByIdUseCase::new(&1, &db_service);
        let data = get_one_dog_fact_by_id_usecase.execute().await.unwrap();

        // then assert the result is the expected entity
        assert_eq!(data.fact_id, 1);
        assert_eq!(data.fact, "fact1");
    }
}
