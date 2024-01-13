use std::net::TcpListener;
use actix_web::dev::Server;
use eureka_web_api::{run, spawn_app};

#[tokio::test]
async fn health_check_works() {
    let port: String = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &port))
        .send()
        .await
        .expect("reqwest execution failed");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
