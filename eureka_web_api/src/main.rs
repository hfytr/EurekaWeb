use eureka_web_api::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener: TcpListener = TcpListener::bind("http://127.0.0.1:8080").expect("failed to bind port");
    run(listener)?.await
}
