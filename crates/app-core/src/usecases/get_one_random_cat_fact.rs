use crate::{services::DatabaseService, utils::error_handling_utils::ErrorHandlingUtils};
use app_domain::{entities::CatFactEntity, error::AppError};

pub struct GetOneRandomCatFactUseCase<'a> {
    service: &'a dyn DatabaseService,
}

impl<'a> GetOneRandomCatFactUseCase<'a> {
    pub fn new(service: &'a dyn DatabaseService) -> Self {
        GetOneRandomCatFactUseCase { service }
    }
}

impl<'a> GetOneRandomCatFactUseCase<'a> {
    pub async fn execute(&self) -> Result<CatFactEntity, AppError> {
        let cat_fact = {
            let mut repo = self.service.get_repo().await.unwrap(); //FIXME
            let fact = repo.get_random_cat_fact().await;
            // transaction is dropped if repo gets out of scope without commit
            repo.commit().await.unwrap(); //FIXME
            fact
        };

        match cat_fact {
            Ok(fact) => Ok(fact),
            Err(_e) => Err(ErrorHandlingUtils::business_error(
                "Cannot get random cat fact",
                None, //FIXME Some(e),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use std::io::{Error, ErrorKind};

    use crate::{
        services::{MockDatabaseService, MockDatabaseServiceRepo},
        usecases::get_one_random_cat_fact::GetOneRandomCatFactUseCase,
    };

    // #[actix_rt::test]
    // async fn test_should_return_generic_message_when_unexpected_repo_error() {
    //     // given the "all cat facts" usecase repo with an unexpected error
    //     let mut db_service = MockDatabaseService::new();
    //     db_service.expect_get_repo().with().times(1).returning(|| {
    //         let mut db_service_repo = MockDatabaseServiceRepo::new();
    //         db_service_repo
    //             .expect_get_random_cat_fact()
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
    //     let get_one_random_cat_fact_usecase = GetOneRandomCatFactUseCase::new(&db_service);
    //     let data = get_one_random_cat_fact_usecase.execute().await;

    //     // then exception
    //     assert!(data.is_err());
    //     let result = data.unwrap_err();
    //     assert_eq!("Cannot get random cat fact", result.message);
    // }

    #[actix_rt::test]
    async fn test_should_return_one_result() {
        // given the "one random cat fact" usecase repo returning one result
        let mut db_service = MockDatabaseService::new();
        db_service.expect_get_repo().with().times(1).returning(|| {
            let mut db_service_repo = MockDatabaseServiceRepo::new();
            db_service_repo
                .expect_get_random_cat_fact()
                .with()
                .times(1)
                .returning(|| {
                    Ok(CatFactEntity {
                        fact_txt: String::from("fact1"),
                        fact_id: 1,
                    })
                });
            db_service_repo
                .expect_commit()
                .times(1)
                .returning(|| Ok(()));
            Ok(Box::new(db_service_repo))
        });

        // when calling usecase
        let get_one_random_cat_fact_usecase = GetOneRandomCatFactUseCase::new(&db_service);
        let data = get_one_random_cat_fact_usecase.execute().await.unwrap();

        // then assert the result is the expected entity
        assert_eq!(data.fact_txt, "fact1");
        assert_eq!(data.fact_id, 1);
    }
}
