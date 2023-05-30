use std::marker::PhantomData;

use crate::{
    services::{DBDogRepo, Persistence, Transaction},
    utils::error_handling_utils::ErrorHandlingUtils,
};
use app_domain::{entities::DogFactEntity, error::AppError};

pub struct GetOneDogFactByIdUseCase<P, R> {
    persistance: P,
    repo: PhantomData<R>,
}

impl<P, DR> GetOneDogFactByIdUseCase<P, DR> {
    pub fn new(persistance: P, repo: PhantomData<DR>) -> Self {
        GetOneDogFactByIdUseCase { persistance, repo }
    }
}

impl<'a, P, DR> GetOneDogFactByIdUseCase<P, DR>
where
    P: Persistence<'a>,
    <P as Persistence<'a>>::Transaction: Transaction,
    DR: DBDogRepo<'a, P>,
{
    pub async fn execute(&self, dog_fact_id: &i32) -> Result<DogFactEntity, AppError> {
        let dog_fact = {
            let mut tx = self.persistance.get_transaction().await.unwrap(); //FIXME
            let fact = DR::get_dog_fact_by_id(&mut tx, *dog_fact_id)
                .await
                .map_err(|_| {
                    ErrorHandlingUtils::business_error(
                        "Cannot get single dog fact",
                        None, //FIXME Some(e),
                    )
                })?;
            // transaction is dropped if repo gets out of scope without commit
            tx.commit().await.unwrap(); //FIXME
            fact
        };

        Ok(dog_fact)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use mockall::predicate::eq;
    // use std::io::{Error, ErrorKind};

    // use crate::services::{MockDatabaseService, MockDatabaseServiceRepo};
    // use app_domain::entities::DogFactEntity;

    // #[actix_rt::test]
    // async fn test_should_return_error_with_generic_message_when_unexpected_repo_error() {
    //     // given the "all dog facts" usecase repo with an unexpected random error
    //     let mut db_service = MockDatabaseService::new();
    //     db_service.expect_get_repo().with().times(1).returning(|| {
    //         let mut db_service_repo = MockDatabaseServiceRepo::new();
    //         db_service_repo
    //             .expect_get_dog_fact_by_id()
    //             .with(eq(1))
    //             .times(1)
    //             .returning(|_| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));
    //         db_service_repo
    //             .expect_commit()
    //             .times(1)
    //             .returning(|| Ok(()));
    //         Ok(Box::new(db_service_repo))
    //     });

    //     // when calling usecase
    //     let get_one_dog_fact_by_id_usecase = GetOneDogFactByIdUseCase::new(&1, &db_service);
    //     let data = get_one_dog_fact_by_id_usecase.execute().await;

    //     // then exception
    //     assert!(data.is_err());
    //     let result = data.unwrap_err();
    //     assert_eq!("Cannot get single dog fact", result.message);
    // }

    // #[actix_rt::test]
    // async fn test_should_return_one_result() {
    //     // given the "one dog fact by id" usecase repo returning one result
    //     let mut db_service = MockDatabaseService::new();
    //     db_service.expect_get_repo().with().times(1).returning(|| {
    //         let mut db_service_repo = MockDatabaseServiceRepo::new();
    //         db_service_repo
    //             .expect_get_dog_fact_by_id()
    //             .with(eq(1))
    //             .times(1)
    //             .returning(|_| {
    //                 Ok(DogFactEntity {
    //                     fact_id: 1,
    //                     fact: String::from("fact1"),
    //                 })
    //             });
    //         db_service_repo
    //             .expect_commit()
    //             .times(1)
    //             .returning(|| Ok(()));
    //         Ok(Box::new(db_service_repo))
    //     });

    //     // when calling usecase
    //     let get_one_dog_fact_by_id_usecase = GetOneDogFactByIdUseCase::new(&1, &db_service);
    //     let data = get_one_dog_fact_by_id_usecase.execute().await.unwrap();

    //     // then assert the result is the expected entity
    //     assert_eq!(data.fact_id, 1);
    //     assert_eq!(data.fact, "fact1");
    // }
}
