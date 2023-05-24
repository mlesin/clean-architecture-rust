use crate::utils::utils_setup::{setup, spawn_app};
use presenter_rest::cat_facts::cat_facts_presenters::CatFactPresenter;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

#[sqlx::test(migrations = "../../migrations")]
async fn test_should_return_multiple_results(_opts: PgPoolOptions, connopts: PgConnectOptions) {
    let db_name = connopts
        .get_database()
        .expect("Can't get test database name");
    // setup (along with fake api for http spi)
    setup(db_name.to_string()).await;
    let api_address = spawn_app(db_name);

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
    assert_eq!(content_json[0].nb_chars, 91);
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_should_return_one_results_only(_opts: PgPoolOptions, connopts: PgConnectOptions) {
    let db_name = connopts
        .get_database()
        .expect("Can't get test database name");
    // setup (along with fake api for http spi)
    setup(db_name.to_string()).await;
    let api_address = spawn_app(db_name);

    // given the "random cat fact" route
    // when getting
    let response = reqwest::get(&format!("{}/api/v1/cats/random", &api_address))
        .await
        .expect("Failed to execute request.");

    // then expect 1 only
    assert!(response.status().is_success());

    let content_json = response.json::<CatFactPresenter>().await.unwrap();

    assert_eq!(content_json.fact, "In the 1930s, two Russian biologists discovered that color change in Siamese kittens depend on their body temperature. Siamese cats carry albino genes that work only when the body temperature is above 98° F. If these kittens are left in a very warm room, their points won’t darken and they will stay a creamy white.");
    assert_eq!(content_json.nb_chars, 315);
}
