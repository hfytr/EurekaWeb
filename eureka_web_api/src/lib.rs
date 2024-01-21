use actix_web::{dev::Server, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use std::net::TcpListener;
mod engine;
use engine::search;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let server: Server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("localhost:3000")
            .allowed_methods(vec!["GET", "POST"]);
            // .allowed_headers(vec![]);
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/search", web::get().to(search))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

pub fn spawn_app() -> String {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").expect("failed to bind port");
    let port: u16 = listener.local_addr().unwrap().port();
    let server: Server = run(listener).expect("run() failed");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
