use std::{env, net::TcpListener};

use actix_web::{dev::Server, middleware::Logger};
use actix_web::{rt, web, App, HttpServer};
use gateway_http::{cat_facts_gateway::CatFactsgatewayHTTP, connection::HttpConnection};
use gateway_pg::{connection::DbConnection, dog_facts_gateway::DogFactsgatewayDB};
use presenter_rest::shared::app_state::AppState;

pub fn setup(
    listener: TcpListener,
    db_name: String,
    cats_source: String,
) -> Result<Server, std::io::Error> {
    let _ = env_logger::try_init(); //.expect("Environment error");

    let db_connection = DbConnection { db_name };
    let http_connection = HttpConnection {};

    let data = web::Data::new(AppState {
        app_name: String::from("Animal Facts API"),
        cats_gateway: Box::new(CatFactsgatewayHTTP {
            http_connection,
            source: cats_source,
        }),
        dogs_gateway: Box::new(DogFactsgatewayDB { db_connection }),
    });

    let port = listener.local_addr().unwrap().to_string();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(Logger::default())
            .configure(presenter_rest::shared::routes::routes)
    })
    .listen(listener)?
    .run();

    println!("Server started on http://{}", port);

    Ok(server)
}

pub fn run(listener: TcpListener) -> Result<(), std::io::Error> {
    let environment_file;
    if let Ok(e) = env::var("ENV") {
        environment_file = format!(".env.{}", e);
    } else {
        environment_file = String::from(".env");
    }

    dotenv::from_filename(environment_file).ok();

    let db_name = dotenv::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    let cats_source = dotenv::var("CATS_SOURCE").expect("CATS_SOURCE must be set");

    rt::System::new().block_on(setup(listener, db_name, cats_source)?)
}
