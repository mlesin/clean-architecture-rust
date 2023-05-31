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
    pub fn new(persistance: P) -> Self {
        GetAllDogFactsUseCase {
            persistance,
            repo: PhantomData::<DR>,
        }
    }
}

impl<P, DR> GetAllDogFactsUseCase<P, DR>
where
    P: Persistence,
    <P as Persistence>::Transaction: Transaction,
    DR: DBDogRepo<P>,
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
    use super::*;
    use lazy_static::lazy_static;
    use std::sync::{Mutex, MutexGuard};

    use crate::services::{MockDBDogRepo, MockPersistence, MockTransaction};

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
    async fn test_should_return_error_with_generic_message_when_unexpected_repo_error() {
        let _m = get_lock(&MTX);

        let mut persistence = MockPersistence::new();
        persistence
            .expect_get_transaction()
            .with()
            .times(1)
            .returning(|| Ok(MockTransaction::new()));

        // given the "all dog facts" usecase repo with an unexpected random error
        let repo_ctx = MockDBDogRepo::<MockPersistence>::get_all_dog_facts_context();
        repo_ctx
            .expect()
            .returning(|_tx| Err(crate::services::Error::DatabaseError));

        // when calling usecase
        let get_all_dog_facts_usecase = GetAllDogFactsUseCase::<
            MockPersistence,
            MockDBDogRepo<MockPersistence>,
        >::new(persistence);
        let data = get_all_dog_facts_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get all dog facts", result.message);
    }

    #[actix_rt::test]
    async fn test_should_return_empty_list() {
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

        // given the "all dog facts" usecase repo returning an empty list
        let repo_ctx = MockDBDogRepo::<MockPersistence>::get_all_dog_facts_context();
        repo_ctx
            .expect()
            .returning(|_tx| Ok(Vec::<DogFactEntity>::new()));

        // when calling usecase
        let get_all_dog_facts_usecase = GetAllDogFactsUseCase::<
            MockPersistence,
            MockDBDogRepo<MockPersistence>,
        >::new(persistence);
        let data = get_all_dog_facts_usecase.execute().await.unwrap();

        // then assert the result is an empty list
        assert_eq!(data.len(), 0);
    }

    #[actix_rt::test]
    async fn test_should_return_list() {
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

        // given the "all dog facts" usecase repo returning a list of 2 entities
        let repo_ctx = MockDBDogRepo::<MockPersistence>::get_all_dog_facts_context();
        repo_ctx.expect().returning(|_tx| {
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

        // when calling usecase
        let get_all_dog_facts_usecase = GetAllDogFactsUseCase::<
            MockPersistence,
            MockDBDogRepo<MockPersistence>,
        >::new(persistence);
        let data = get_all_dog_facts_usecase.execute().await.unwrap();

        // then assert the result is an empty list
        assert_eq!(data.len(), 2);
    }
}
