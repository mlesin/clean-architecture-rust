use std::{env, net::TcpListener};

use actix_web::{dev::Server, middleware::Logger};
use actix_web::{web, App, HttpServer};
use gateway_http::{cat_facts_gateway::CatFactsgatewayHTTP, connection::HttpConnection};
use gateway_pg::{connection::DbConnection, dog_facts_gateway::DogFactsgatewayDB};
use presenter_rest::shared::app_state::AppState;

pub fn server(listener: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug");

    let _ = env_logger::try_init(); //.expect("Environment error");

    let db_connection = DbConnection { db_name: db_name.to_string() };
    let http_connection = HttpConnection {};

    let data = web::Data::new(AppState {
        app_name: String::from("Animal Facts API"),
        cats_gateway: Box::new(CatFactsgatewayHTTP {
            http_connection,
            source: dotenv::var("CATS_SOURCE").expect("CATS_SOURCE must be set"),
        }),
        dogs_gateway: Box::new(DogFactsgatewayDB { db_connection }),
    });

    let port = listener.local_addr().unwrap().port();

    let server = HttpServer::new(move || App::new().app_data(data.clone()).wrap(Logger::default()).configure(presenter_rest::shared::routes::routes))
        .listen(listener)?
        .run();

    println!("Server running on port {}, db_name {}", port, db_name);

    Ok(server)
}
