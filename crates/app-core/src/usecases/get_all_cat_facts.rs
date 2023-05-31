use std::marker::PhantomData;

use crate::services::{CatRepo, Persistence, Transaction};
use app_domain::entities::CatFactEntity;

use super::UseCaseError;

pub struct GetAllCatFactsUseCase<P, R> {
    persistance: P,
    repo: PhantomData<R>,
}

impl<P, CR> GetAllCatFactsUseCase<P, CR> {
    pub fn new(persistance: P) -> Self {
        GetAllCatFactsUseCase {
            persistance,
            repo: PhantomData::<CR>,
        }
    }
}

impl<P, CR> GetAllCatFactsUseCase<P, CR>
where
    P: Persistence,
    <P as Persistence>::Transaction: Transaction,
    CR: CatRepo<P>,
{
    pub async fn execute(&self) -> Result<Vec<CatFactEntity>, UseCaseError> {
        let cat_facts = {
            let mut tx = self.persistance.get_transaction().await?;
            let facts = CR::get_all_cat_facts(&mut tx).await?;
            tx.commit().await?;
            facts
        };

        Ok(cat_facts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use std::sync::{Mutex, MutexGuard};

    use crate::services::{MockCatRepo, MockPersistence, MockTransaction};

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

    type MockRepo = MockCatRepo<MockPersistence>;
    type MockUseCase = GetAllCatFactsUseCase<MockPersistence, MockRepo>;

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
        let repo_ctx = MockRepo::get_all_cat_facts_context();
        repo_ctx
            .expect()
            .returning(|_tx| Err(crate::services::RepositoryError("Oh no!".into())));

        // when calling usecase
        let get_all_cat_facts_usecase = MockUseCase::new(persistence);
        let data = get_all_cat_facts_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Repository error: Oh no!", result.to_string());
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

        // given the "all cat facts" usecase repo returning an empty list
        let repo_ctx = MockRepo::get_all_cat_facts_context();
        repo_ctx
            .expect()
            .returning(|_tx| Ok(Vec::<CatFactEntity>::new()));

        // when calling usecase
        let get_all_cat_facts_usecase = MockUseCase::new(persistence);
        let data = get_all_cat_facts_usecase.execute().await.unwrap();

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

        // given the "all cat facts" usecase repo returning a list of 2 entities
        let repo_ctx = MockRepo::get_all_cat_facts_context();
        repo_ctx.expect().returning(|_tx| {
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

        // when calling usecase
        let get_all_cat_facts_usecase = MockUseCase::new(persistence);
        let data = get_all_cat_facts_usecase.execute().await.unwrap();

        // then assert the result is an empty list
        assert_eq!(data.len(), 2);
    }
}
