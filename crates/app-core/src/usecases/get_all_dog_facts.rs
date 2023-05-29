use crate::{
    services::{DBDogRepo, DatabaseService, SharedPersistence},
    utils::error_handling_utils::ErrorHandlingUtils,
};
use app_domain::{entities::DogFactEntity, error::AppError};

pub struct GetAllDogFactsUseCaseA<'a> {
    persistance: SharedPersistence,
    repo: &'a dyn DBDogRepo,
}

impl<'a> GetAllDogFactsUseCaseA<'a> {
    pub fn new(persistance: SharedPersistence, repo: &'a dyn DBDogRepo) -> Self {
        GetAllDogFactsUseCaseA { persistance, repo }
    }
}

impl<'a> GetAllDogFactsUseCaseA<'a> {
    pub async fn execute(&'a self) -> Result<Vec<DogFactEntity>, AppError> {
        let dog_facts = {
            let mut conn = self.persistance.get_connection().await.unwrap(); //FIXME
            let mut tx = conn.start_transaction().unwrap(); //FIXME
            let facts = self.repo.get_all_dog_facts(&mut *tx).await;
            // transaction is dropped if repo gets out of scope without commit
            tx.commit().unwrap(); //FIXME
            facts
        };

        match dog_facts {
            Ok(facts) => Ok(facts),
            Err(_e) => Err(ErrorHandlingUtils::business_error(
                "Cannot get all dog facts",
                None, //FIXME Some(e),
            )),
        }
    }
}

pub struct GetAllDogFactsUseCase<'a> {
    service: &'a dyn DatabaseService,
}

impl<'a> GetAllDogFactsUseCase<'a> {
    pub fn new(service: &'a dyn DatabaseService) -> Self {
        GetAllDogFactsUseCase { service }
    }
}

impl<'a> GetAllDogFactsUseCase<'a> {
    pub async fn execute(&self) -> Result<Vec<DogFactEntity>, AppError> {
        let dog_facts = {
            let mut repo = self.service.get_repo().await.unwrap(); //FIXME
            let facts = repo.get_all_dog_facts().await;
            // transaction is dropped if repo gets out of scope without commit
            repo.commit().await.unwrap(); //FIXME
            facts
        };

        match dog_facts {
            Ok(facts) => Ok(facts),
            Err(_e) => Err(ErrorHandlingUtils::business_error(
                "Cannot get all dog facts",
                None, //FIXME Some(e),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use std::io::{Error, ErrorKind};

    use crate::services::{MockDatabaseService, MockDatabaseServiceRepo};
    use app_domain::entities::DogFactEntity;

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

    #[actix_rt::test]
    async fn test_should_return_empty_list() {
        // given the "all dog facts" usecase repo returning an empty list
        let mut db_service = MockDatabaseService::new();
        db_service.expect_get_repo().with().times(1).returning(|| {
            let mut db_service_repo = MockDatabaseServiceRepo::new();
            db_service_repo
                .expect_get_all_dog_facts()
                .with()
                .times(1)
                .returning(|| Ok(Vec::<DogFactEntity>::new()));
            db_service_repo
                .expect_commit()
                .times(1)
                .returning(|| Ok(()));
            Ok(Box::new(db_service_repo))
        });

        // when calling usecase
        let get_all_dog_facts_usecase = GetAllDogFactsUseCase::new(&db_service);
        let data = get_all_dog_facts_usecase.execute().await.unwrap();

        // then assert the result is an empty list
        assert_eq!(data.len(), 0);
    }

    #[actix_rt::test]
    async fn test_should_return_list() {
        // given the "all dog facts" usecase repo returning a list of 2 entities
        let mut db_service = MockDatabaseService::new();
        db_service.expect_get_repo().with().times(1).returning(|| {
            let mut db_service_repo = MockDatabaseServiceRepo::new();
            db_service_repo
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
            db_service_repo
                .expect_commit()
                .times(1)
                .returning(|| Ok(()));
            Ok(Box::new(db_service_repo))
        });

        // when calling usecase
        let get_all_dog_facts_usecase = GetAllDogFactsUseCase::new(&db_service);
        let data = get_all_dog_facts_usecase.execute().await.unwrap();

        // then assert the result is an empty list
        assert_eq!(data.len(), 2);
    }
}
