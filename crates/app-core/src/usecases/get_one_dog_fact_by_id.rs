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
    pub fn new(persistance: P) -> Self {
        GetOneDogFactByIdUseCase {
            persistance,
            repo: PhantomData::<DR>,
        }
    }
}

impl<P, DR> GetOneDogFactByIdUseCase<P, DR>
where
    P: Persistence,
    <P as Persistence>::Transaction: Transaction,
    DR: DBDogRepo<P>,
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
        let repo_ctx = MockDBDogRepo::<MockPersistence>::get_dog_fact_by_id_context();
        repo_ctx
            .expect()
            .times(1)
            .returning(|_tx, _id| Err(crate::services::Error::DatabaseError));

        // when calling usecase
        let get_one_dog_fact_by_id_usecase = GetOneDogFactByIdUseCase::<
            MockPersistence,
            MockDBDogRepo<MockPersistence>,
        >::new(persistence);
        let data = get_one_dog_fact_by_id_usecase.execute(&1).await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get single dog fact", result.message);
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

        // given the "one dog fact by id" usecase repo returning one result
        let repo_ctx = MockDBDogRepo::<MockPersistence>::get_dog_fact_by_id_context();
        repo_ctx.expect().times(1).returning(|_tx, _id| {
            Ok(DogFactEntity {
                fact_id: 1,
                fact: String::from("fact1"),
            })
        });

        // when calling usecase
        let get_one_dog_fact_by_id_usecase = GetOneDogFactByIdUseCase::<
            MockPersistence,
            MockDBDogRepo<MockPersistence>,
        >::new(persistence);
        let data = get_one_dog_fact_by_id_usecase.execute(&1).await.unwrap();

        // then assert the result is the expected entity
        assert_eq!(data.fact_id, 1);
        assert_eq!(data.fact, "fact1");
    }
}
