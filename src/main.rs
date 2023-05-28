use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8888").expect("Failed to bind random port");
    main_web::run(listener)
}
