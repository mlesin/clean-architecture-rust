use std::marker::PhantomData;

use crate::{
    services::{DBCatRepo, Persistence, Transaction},
    utils::error_handling_utils::ErrorHandlingUtils,
};
use app_domain::{entities::CatFactEntity, error::AppError};

pub struct GetOneRandomCatFactUseCase<P, R> {
    persistance: P,
    repo: PhantomData<R>,
}

impl<P, CR> GetOneRandomCatFactUseCase<P, CR> {
    pub fn new(persistance: P, repo: PhantomData<CR>) -> Self {
        GetOneRandomCatFactUseCase { persistance, repo }
    }
}

impl<'a, P, CR> GetOneRandomCatFactUseCase<P, CR>
where
    P: Persistence<'a>,
    <P as Persistence<'a>>::Transaction: Transaction,
    CR: DBCatRepo<'a, P>,
{
    pub async fn execute(&self) -> Result<CatFactEntity, AppError> {
        let cat_fact = {
            let mut tx = self.persistance.get_transaction().await.unwrap(); //FIXME
            let fact = CR::get_random_cat_fact(&mut tx).await.map_err(|_| {
                ErrorHandlingUtils::business_error(
                    "Cannot get random cat fact",
                    None, //FIXME Some(e),
                )
            })?;
            // transaction is dropped if repo gets out of scope without commit
            tx.commit().await.unwrap(); //FIXME
            fact
        };

        Ok(cat_fact)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use std::io::{Error, ErrorKind};

    // use crate::{
    //     services::{MockDatabaseService, MockDatabaseServiceRepo},
    //     usecases::get_one_random_cat_fact::GetOneRandomCatFactUseCase,
    // };

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

    // #[actix_rt::test]
    // async fn test_should_return_one_result() {
    //     // given the "one random cat fact" usecase repo returning one result
    //     let mut db_service = MockDatabaseService::new();
    //     db_service.expect_get_repo().with().times(1).returning(|| {
    //         let mut db_service_repo = MockDatabaseServiceRepo::new();
    //         db_service_repo
    //             .expect_get_random_cat_fact()
    //             .with()
    //             .times(1)
    //             .returning(|| {
    //                 Ok(CatFactEntity {
    //                     fact_txt: String::from("fact1"),
    //                     fact_id: 1,
    //                 })
    //             });
    //         db_service_repo
    //             .expect_commit()
    //             .times(1)
    //             .returning(|| Ok(()));
    //         Ok(Box::new(db_service_repo))
    //     });

    //     // when calling usecase
    //     let get_one_random_cat_fact_usecase = GetOneRandomCatFactUseCase::new(&db_service);
    //     let data = get_one_random_cat_fact_usecase.execute().await.unwrap();

    //     // then assert the result is the expected entity
    //     assert_eq!(data.fact_txt, "fact1");
    //     assert_eq!(data.fact_id, 1);
    // }
}
