use async_trait::async_trait;

use crate::{
    services::DatabaseService, usecases::interfaces::UseCase,
    utils::error_handling_utils::ErrorHandlingUtils,
};
use app_domain::{entities::CatFactEntity, error::ApiError};

pub struct GetAllCatFactsUseCase<'a> {
    service: &'a dyn DatabaseService,
}

impl<'a> GetAllCatFactsUseCase<'a> {
    pub fn new(service: &'a dyn DatabaseService) -> Self {
        GetAllCatFactsUseCase { service }
    }
}

#[async_trait(?Send)]
impl<'a> UseCase<Vec<CatFactEntity>> for GetAllCatFactsUseCase<'a> {
    async fn execute(&self) -> Result<Vec<CatFactEntity>, ApiError> {
        let cat_facts = {
            let mut repo = self.service.get_repo().await.unwrap(); //FIXME
            let facts = repo.get_all_cat_facts().await;
            // transaction is dropped if repo gets out of scope without commit
            repo.commit().await.unwrap(); //FIXME
            facts
        };

        match cat_facts {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::business_error(
                "Cannot get all cat facts",
                Some(e),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    use crate::services::{MockDatabaseService, MockDatabaseServiceRepo};

    #[actix_rt::test]
    async fn test_should_return_generic_message_when_unexpected_repo_error() {
        // given the "all cat facts" usecase repo with an unexpected error
        let mut db_service = MockDatabaseService::new();
        db_service.expect_get_repo().with().times(1).returning(|| {
            let mut db_service_repo = MockDatabaseServiceRepo::new();
            db_service_repo
                .expect_get_all_cat_facts()
                .with()
                .times(1)
                .returning(|| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));
            db_service_repo
                .expect_commit()
                .times(1)
                .returning(|| Ok(()));
            Ok(Box::new(db_service_repo))
        });

        // when calling usecase
        let get_all_cat_facts_usecase = GetAllCatFactsUseCase::new(&db_service);
        let data = get_all_cat_facts_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get all cat facts", result.message);
    }

    #[actix_rt::test]
    async fn test_should_return_empty_list() {
        // given the "all cat facts" usecase repo returning an empty list
        let mut db_service = MockDatabaseService::new();
        db_service.expect_get_repo().with().times(1).returning(|| {
            let mut db_service_repo = MockDatabaseServiceRepo::new();
            db_service_repo
                .expect_get_all_cat_facts()
                .with()
                .times(1)
                .returning(|| Ok(Vec::<CatFactEntity>::new()));
            db_service_repo
                .expect_commit()
                .times(1)
                .returning(|| Ok(()));
            Ok(Box::new(db_service_repo))
        });

        // when calling usecase
        let get_all_cat_facts_usecase = GetAllCatFactsUseCase::new(&db_service);
        let data = get_all_cat_facts_usecase.execute().await.unwrap();

        // then assert the result is an empty list
        assert_eq!(data.len(), 0);
    }

    #[actix_rt::test]
    async fn test_should_return_list() {
        // given the "all cat facts" usecase repo returning a list of 2 entities
        let mut db_service = MockDatabaseService::new();
        db_service.expect_get_repo().with().times(1).returning(|| {
            let mut db_service_repo = MockDatabaseServiceRepo::new();
            db_service_repo
                .expect_get_all_cat_facts()
                .with()
                .times(1)
                .returning(|| {
                    Ok(vec![
                        CatFactEntity {
                            fact_txt: String::from("fact1"),
                            fact_id: 1,
                        },
                        CatFactEntity {
                            fact_txt: String::from("fact2"),
                            fact_id: 2,
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
        let get_all_cat_facts_usecase = GetAllCatFactsUseCase::new(&db_service);
        let data = get_all_cat_facts_usecase.execute().await.unwrap();

        // then assert the result is an empty list
        assert_eq!(data.len(), 2);
    }
}
