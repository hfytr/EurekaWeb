use eureka_web_api::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").expect("failed to bind port");
    run(listener)?.await
}
