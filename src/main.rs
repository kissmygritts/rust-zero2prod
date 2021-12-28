use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4444")
        .expect("Failed to bind to port 4444");
    run(listener)?.await
}