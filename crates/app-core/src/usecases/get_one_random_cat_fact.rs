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
    pub fn new(persistance: P) -> Self {
        GetOneRandomCatFactUseCase {
            persistance,
            repo: PhantomData::<CR>,
        }
    }
}

impl<P, CR> GetOneRandomCatFactUseCase<P, CR>
where
    P: Persistence,
    <P as Persistence>::Transaction: Transaction,
    CR: DBCatRepo<P>,
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
    use super::*;
    use lazy_static::lazy_static;
    use std::sync::{Mutex, MutexGuard};

    use crate::services::{MockDBCatRepo, MockPersistence, MockTransaction};

    lazy_static! {
        static ref MTX: Mutex<()> = Mutex::new(());
    }

    // When a test panics, it will poison the Mutex. Since we don't actually
    // care about the state of the data we ignore that it is poisoned and grab
    // the lock regardless.  If you just do `let _m = &MTX.lock().unwrap()`, one
    // test panicking will cause all other tests that try and acquire a lock on
    // that Mutex to also panic.
    fn get_lock(m: &'static Mutex<()>) -> MutexGuard<'static, ()> {
        match m.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    #[actix_rt::test]
    async fn test_should_return_generic_message_when_unexpected_repo_error() {
        let _m = get_lock(&MTX);

        let mut persistence = MockPersistence::new();
        persistence
            .expect_get_transaction()
            .with()
            .times(1)
            .returning(|| Ok(MockTransaction::new()));

        // given the "all cat facts" usecase repo with an unexpected error
        let repo_ctx = MockDBCatRepo::<MockPersistence>::get_random_cat_fact_context();
        repo_ctx
            .expect()
            .returning(|_tx| Err(crate::services::Error::DatabaseError));

        // when calling usecase
        let get_one_random_cat_fact_usecase = GetOneRandomCatFactUseCase::<
            MockPersistence,
            MockDBCatRepo<MockPersistence>,
        >::new(persistence);
        let data = get_one_random_cat_fact_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get random cat fact", result.message);
    }

    #[actix_rt::test]
    async fn test_should_return_one_result() {
        let _m = get_lock(&MTX);

        let mut persistence = MockPersistence::new();
        persistence
            .expect_get_transaction()
            .with()
            .times(1)
            .returning(|| {
                let mut tx = MockTransaction::new();
                tx.expect_commit().times(1).returning(|| Ok(()));
                Ok(tx)
            });

        // given the "one random cat fact" usecase repo returning one result
        let repo_ctx = MockDBCatRepo::<MockPersistence>::get_random_cat_fact_context();
        repo_ctx.expect().returning(|_tx| {
            Ok(CatFactEntity {
                fact_txt: String::from("fact1"),
                fact_id: 1,
            })
        });

        // when calling usecase
        let get_one_random_cat_fact_usecase = GetOneRandomCatFactUseCase::<
            MockPersistence,
            MockDBCatRepo<MockPersistence>,
        >::new(persistence);
        let data = get_one_random_cat_fact_usecase.execute().await.unwrap();

        // then assert the result is the expected entity
        assert_eq!(data.fact_txt, "fact1");
        assert_eq!(data.fact_id, 1);
    }
}
