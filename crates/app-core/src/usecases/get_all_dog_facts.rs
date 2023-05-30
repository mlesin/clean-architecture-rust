use std::marker::PhantomData;

use crate::{
    services::{DBDogRepo, Persistence, Transaction},
    utils::error_handling_utils::ErrorHandlingUtils,
};
use app_domain::{entities::DogFactEntity, error::AppError};

pub struct GetAllDogFactsUseCase<P, R> {
    persistance: P,
    repo: PhantomData<R>,
}

impl<P, DR> GetAllDogFactsUseCase<P, DR> {
    pub fn new(persistance: P, repo: PhantomData<DR>) -> Self {
        GetAllDogFactsUseCase { persistance, repo }
    }
}

impl<'a, P, DR> GetAllDogFactsUseCase<P, DR>
where
    P: Persistence<'a>,
    <P as Persistence<'a>>::Transaction: Transaction,
    DR: DBDogRepo<'a, P>,
{
    pub async fn execute(&self) -> Result<Vec<DogFactEntity>, AppError> {
        let dog_facts = {
            let mut tx = self.persistance.get_transaction().await.unwrap(); //FIXME

            // let mut tx = conn.start_transaction().unwrap(); //FIXME
            let facts = DR::get_all_dog_facts(&mut tx).await.map_err(|_| {
                ErrorHandlingUtils::business_error(
                    "Cannot get all dog facts",
                    None, //FIXME Some(e),
                )
            })?;
            // transaction is dropped if repo gets out of scope without commit
            tx.commit().await.unwrap(); //FIXME
            facts
        };

        Ok(dog_facts)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
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
    //             .expect_get_all_dog_facts()
    //             .with()
    //             .times(1)
    //             .returning(|| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));
    //         db_service_repo
    //             .expect_commit()
    //             .times(1)
    //             .returning(|| Ok(()));
    //         Ok(Box::new(db_service_repo))
    //     });

    //     // when calling usecase
    //     let get_all_dog_facts_usecase = GetAllDogFactsUseCase::new(&db_service);
    //     let data = get_all_dog_facts_usecase.execute().await;

    //     // then exception
    //     assert!(data.is_err());
    //     let result = data.unwrap_err();
    //     assert_eq!("Cannot get all dog facts", result.message);
    // }

    // #[actix_rt::test]
    // async fn test_should_return_empty_list() {
    //     // given the "all dog facts" usecase repo returning an empty list
    //     let mut db_service = MockDatabaseService::new();
    //     db_service.expect_get_repo().with().times(1).returning(|| {
    //         let mut db_service_repo = MockDatabaseServiceRepo::new();
    //         db_service_repo
    //             .expect_get_all_dog_facts()
    //             .with()
    //             .times(1)
    //             .returning(|| Ok(Vec::<DogFactEntity>::new()));
    //         db_service_repo
    //             .expect_commit()
    //             .times(1)
    //             .returning(|| Ok(()));
    //         Ok(Box::new(db_service_repo))
    //     });

    //     // when calling usecase
    //     let get_all_dog_facts_usecase = GetAllDogFactsUseCase::new(&db_service);
    //     let data = get_all_dog_facts_usecase.execute().await.unwrap();

    //     // then assert the result is an empty list
    //     assert_eq!(data.len(), 0);
    // }

    // #[actix_rt::test]
    // async fn test_should_return_list() {
    //     // given the "all dog facts" usecase repo returning a list of 2 entities
    //     let mut db_service = MockDatabaseService::new();
    //     db_service.expect_get_repo().with().times(1).returning(|| {
    //         let mut db_service_repo = MockDatabaseServiceRepo::new();
    //         db_service_repo
    //             .expect_get_all_dog_facts()
    //             .with()
    //             .times(1)
    //             .returning(|| {
    //                 Ok(vec![
    //                     DogFactEntity {
    //                         fact_id: 1,
    //                         fact: String::from("fact1"),
    //                     },
    //                     DogFactEntity {
    //                         fact_id: 2,
    //                         fact: String::from("fact2"),
    //                     },
    //                 ])
    //             });
    //         db_service_repo
    //             .expect_commit()
    //             .times(1)
    //             .returning(|| Ok(()));
    //         Ok(Box::new(db_service_repo))
    //     });

    //     // when calling usecase
    //     let get_all_dog_facts_usecase = GetAllDogFactsUseCase::new(&db_service);
    //     let data = get_all_dog_facts_usecase.execute().await.unwrap();

    //     // then assert the result is an empty list
    //     assert_eq!(data.len(), 2);
    // }
}
