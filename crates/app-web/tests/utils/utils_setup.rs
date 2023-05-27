use actix_web::{web, App, HttpResponse, HttpServer};
use sqlx::{postgres::PgConnectOptions, ConnectOptions};
use std::net::{TcpListener, TcpStream};

use crate::integration_tests::fixtures::fixtures_run::execute_imports;
use crate::utils::utils_file::read_from_file;
use gateway_http::models::{CatFactApiModel, CatFactsApiModel};

pub async fn spawn_app(connopts: &PgConnectOptions) -> String {
    // Let the OS assign a port (:0)
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();

    let db_name = connopts
        .get_database()
        .expect("Can't get test database name");

    let server = app_web::setup(
        listener,
        db_name.to_string(),
        "http://127.0.0.1:3333".to_string(),
    );

    if TcpStream::connect("127.0.0.1:3333").is_ok() {
        println!("Http source faked server already running");
    } else {
        spawn_http_spi();
    }

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

pub fn spawn_http_spi() -> String {
    async fn facts_route() -> HttpResponse {
        let json =
            read_from_file::<CatFactsApiModel>("tests/integration_tests/fixtures/cat_facts.json")
                .unwrap();
        HttpResponse::Ok().json(json)
    }

    async fn random_fact_route() -> HttpResponse {
        HttpResponse::Ok().json(CatFactApiModel {
            fact: String::from("In the 1930s, two Russian biologists discovered that color change in Siamese kittens depend on their body temperature. Siamese cats carry albino genes that work only when the body temperature is above 98° F. If these kittens are left in a very warm room, their points won’t darken and they will stay a creamy white."),
            length: 315,
        })
    }

    let s1 = HttpServer::new(move || {
        App::new()
            .route("facts", web::get().to(facts_route))
            .route("fact", web::get().to(random_fact_route))
    })
    .bind("127.0.0.1:3333")
    .expect("woops")
    .run();

    let _ = tokio::spawn(s1);

    "http://127.0.0.1:3333".to_string()
}

pub async fn setup(connopts: &PgConnectOptions) {
    let mut db_connection_postgres_db = connopts.connect().await.unwrap();
    execute_imports(&mut db_connection_postgres_db).await;
}
