use std::net::TcpListener;
use eureka_web_api::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0")
        .expect("failed to bind port");
    run(listener)?.await
}
