use std::{env, net::TcpListener};

use actix_web::middleware::Logger;
use actix_web::{rt, web, App, HttpServer};
use presenter_rest::RestAppState;
use service_auth::{cat_facts_service::CatFactsserviceHTTP, connection::HttpConnection};
use service_db::db_service::{CatRepoPG, DogRepoPG, PersistencePG};

pub async fn setup(
    listener: TcpListener,
    db_name: String,
    cats_source: String,
) -> Result<(), std::io::Error> {
    let _ = env_logger::try_init(); //.expect("Environment error");

    let http_connection = HttpConnection {};

    let data = web::Data::new(RestAppState {
        auth_service: Box::new(CatFactsserviceHTTP {
            http_connection,
            source: cats_source,
        }),
        persistence_service: PersistencePG::new(&db_name).await.unwrap(), //FIXME
    });

    let port = listener.local_addr().unwrap().to_string();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(Logger::default())
            .configure(
                presenter_rest::RestControllers::<PersistencePG, DogRepoPG, CatRepoPG>::routes,
            )
    })
    .listen(listener)?
    .run();

    println!("Server started on http://{}", port);

    server.await
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

    rt::System::new().block_on(setup(listener, db_name, cats_source))
}
