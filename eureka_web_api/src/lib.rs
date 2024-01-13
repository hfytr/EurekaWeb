use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;
mod engine;
use engine::EngineCommunicator;
use std::sync::Mutex;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let communicator = web::Data::new(Mutex::new(EngineCommunicator::new()));
    println!("hi from run");
    let server: Server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .app_data(communicator.clone())
            .service(
                web::scope("/search")
                    .route("/test", web::get().to(health_check))
                    .route("/start", web::post().to(EngineCommunicator::start_search))
                    .route("/info", web::get().to(EngineCommunicator::get_info)),
            )
    })
    .listen(listener)?
    .run();
    Ok(server)
}

pub fn spawn_app() -> String {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").expect("failed to bind port");
    let port: u16 = listener.local_addr().unwrap().port();
    let server: Server = run(listener).expect("run() failed");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
