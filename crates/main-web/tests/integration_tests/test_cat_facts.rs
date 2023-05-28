use crate::utils::utils_setup::{setup, spawn_app};
use presenter_rest::cat_facts::cat_facts_presenters::CatFactPresenter;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

#[sqlx::test(migrations = "../service-db/migrations")]
async fn test_should_return_multiple_results(_opts: PgPoolOptions, connopts: PgConnectOptions) {
    // setup
    setup(&connopts).await;
    let api_address = spawn_app(&connopts).await;

    // given the "all cat facts" route

    // when getting
    let response = reqwest::get(&format!("{}/api/v1/cats/", &api_address))
        .await
        .expect("Failed to execute request.");

    // then expect entire list
    assert!(response.status().is_success());

    let content_json = response.json::<Vec<CatFactPresenter>>().await.unwrap();

    assert_eq!(content_json.len(), 10);
    assert_eq!(
        content_json[0].fact,
        "The first true cats came into existence about 12 million years ago and were the Proailurus."
    );
    assert_eq!(content_json[0].id, 1);
}

#[sqlx::test(migrations = "../service-db/migrations")]
async fn test_should_return_one_results_only(_opts: PgPoolOptions, connopts: PgConnectOptions) {
    // setup
    setup(&connopts).await;
    let api_address = spawn_app(&connopts).await;

    // given the "random cat fact" route
    // when getting
    let response = reqwest::get(&format!("{}/api/v1/cats/random", &api_address))
        .await
        .expect("Failed to execute request.");

    // then expect 1 only
    assert!(response.status().is_success());

    let content_json = response.json::<CatFactPresenter>().await.unwrap();

    assert_eq!(content_json.fact, "The first true cats came into existence about 12 million years ago and were the Proailurus.");
    assert_eq!(content_json.id, 1);
}
