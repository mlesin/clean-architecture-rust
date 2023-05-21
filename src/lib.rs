use actix_web::dev::Server;
use std::net::TcpListener;

pub mod infrastructure;

extern crate dotenv;
extern crate log;

pub fn run(listener: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
    infrastructure::server(listener, db_name)
}
