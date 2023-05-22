use crate::utils::utils_setup::{setup, spawn_app};
use presenter_rest::dog_facts::dog_facts_presenters::DogFactPresenter;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

#[sqlx::test(migrations = "../../migrations")]
async fn test_should_return_multiple_results(_opts: PgPoolOptions, connopts: PgConnectOptions) {
    let db_name = connopts.get_database().expect("Can't get test database name");
    // setup
    let _ctx = setup(db_name).await;
    let api_address = spawn_app(db_name);

    // given the "all dog facts" route

    // when getting
    let response = reqwest::get(&format!("{}/api/v1/dogs/", &api_address)).await.expect("Failed to execute request.");

    // then expect 3 results (inserted in db)
    assert!(response.status().is_success());

    let content_json = response.json::<Vec<DogFactPresenter>>().await.unwrap();

    assert_eq!(content_json.len(), 3);
    assert_eq!(content_json[0].txt, "Forty-five percent of U.S. dogs sleep in their owner's bed");
    assert_eq!(content_json[0].fact_id, 1);
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_should_return_one_results_only(_opts: PgPoolOptions, connopts: PgConnectOptions) {
    let db_name = connopts.get_database().expect("Can't get test database name");
    // setup
    let _ctx = setup(db_name).await;
    let api_address = spawn_app(db_name);

    // given the "single dog facts" route
    let dog_fact_id: i8 = 2;

    // when getting
    let response = reqwest::get(&format!("{}/api/v1/dogs/{}", &api_address, &dog_fact_id)).await.expect("Failed to execute request.");

    // then expect 1 result (id 2 inserted in db)
    assert!(response.status().is_success());

    let content_json = response.json::<DogFactPresenter>().await.unwrap();

    assert_eq!(content_json.txt, "Seventy percent of people sign their dog's name on their holiday cards");
    assert_eq!(content_json.fact_id, 2);
}
