mod entities {
    use std::{future::Future, pin::Pin};

    pub type AsyncFn<'a, Arg, Res> =
        dyn Fn(Arg) -> Pin<Box<dyn Future<Output = Result<Res, String>> + Send + 'a>> + Sync + 'a;

    #[derive(Debug, Default, Clone, sqlx::FromRow)]
    pub struct DomainPlayer {
        pub id: String,
        pub shirt_number: i64,
    }

    pub struct PlayerCreateFnArgs<'a> {
        // other needed fields here
        pub shirt_next_value: Box<
            dyn FnMut(String) -> Pin<Box<dyn Future<Output = Result<i64, String>> + Send + 'a>>
                + Send
                + Sync
                + 'a,
        >,
    }
}

mod usecases {
    use std::sync::Arc;

    use super::entities::{AsyncFn, DomainPlayer, PlayerCreateFnArgs};

    #[async_trait::async_trait]
    pub trait PlayerPort: Send + Sync {
        async fn player_create<'a>(
            &'a self,
            _input: &PlayerInput,
            lambda: &AsyncFn<'_, PlayerCreateFnArgs<'a>, DomainPlayer>,
        ) -> Result<DomainPlayer, String>;
    }

    pub struct RepoPorts {
        pub command_pg_repo: Arc<dyn PlayerPort>,
    }

    #[derive(Default)]
    pub struct PlayerInput {
        pub id: String,
    }

    pub struct CreatePlayerUseCase {
        repos: Arc<RepoPorts>,
    }

    impl CreatePlayerUseCase {
        pub fn new(repos: Arc<RepoPorts>) -> Self {
            Self { repos }
        }

        pub async fn execute(&self, input: &PlayerInput) -> Result<DomainPlayer, String> {
            let res = self
                .repos
                .command_pg_repo
                .player_create(&input, &|mut args| {
                    let input = input;

                    Box::pin(async move {
                        let shirt_number = (args.shirt_next_value)("player".to_string()).await?;

                        let o = DomainPlayer {
                            id: input.id.to_string(),
                            shirt_number,
                        };

                        Ok(o)
                    })
                })
                .await?;

            Ok(res)
        }
    }
}

mod adapters {
    use std::{future::Future, pin::Pin, sync::Arc};

    use tokio::sync::Mutex;

    use super::entities::{AsyncFn, DomainPlayer, PlayerCreateFnArgs};
    use super::usecases::{PlayerInput, PlayerPort};

    pub struct PgRepo {
        pool: sqlx::PgPool,
    }

    impl PgRepo {
        pub fn new(pool: sqlx::PgPool) -> Self {
            Self { pool }
        }
    }

    #[async_trait::async_trait]
    impl PlayerPort for PgRepo {
        async fn player_create<'a>(
            &'a self,
            _input: &PlayerInput,
            lambda: &AsyncFn<'_, PlayerCreateFnArgs<'a>, DomainPlayer>,
        ) -> Result<DomainPlayer, String> {
            let tx = Arc::new(Mutex::new(self.pool.begin().await.unwrap()));

            // use _input here

            let shirt_next_value = Box::new({
                let tx = tx.clone();
                move |model: String| -> Pin<Box<dyn Future<Output = Result<i64, std::string::String>> + Send>> {
                let tx = tx.clone();
                Box::pin(async move {
                    self.shirt_get_next_and_increase(&mut *tx.lock().await, model)
                        .await
                })
            }
            });

            let domain_player = lambda(PlayerCreateFnArgs { shirt_next_value }).await?;

            let res = sqlx::query_as::<_, DomainPlayer>(
                "INSERT INTO player (...) VALUES (...) RETURNING *",
            )
            .bind(domain_player.id)
            .bind(domain_player.shirt_number)
            .fetch_one(&mut *tx.lock().await)
            .await
            .unwrap();

            Ok(res)
        }
    }

    impl PgRepo {
        async fn shirt_get_next_and_increase<'a>(
            &'a self,
            _tx: &'a mut sqlx::PgConnection,
            _model: String,
        ) -> Result<i64, String> {
            // Here I'm awaiting an async call for DB operations using the same DB transacion of the caller (_tx)...

            // use _tx here...

            let res = 123;

            Ok(res)
        }
    }
}

use std::sync::Arc;

use adapters::PgRepo;
use usecases::{CreatePlayerUseCase, PlayerInput, RepoPorts};

pub async fn main() -> Result<(), String> {
    let pg_pool = sqlx::PgPool::connect("fake_url").await.unwrap();

    let pg_repo = Arc::new(PgRepo::new(pg_pool));

    let dependencies = Arc::new(RepoPorts {
        command_pg_repo: pg_repo,
    });

    let handler = CreatePlayerUseCase::new(dependencies);

    let new_player_input = PlayerInput {
        id: "abc".to_string(),
    };

    let player = handler.execute(&new_player_input).await?;

    dbg!(player);

    Ok(())
}
