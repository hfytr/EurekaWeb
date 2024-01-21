use eureka_web_api::spawn_app;

#[tokio::test]
async fn health_check_works() {
    let address: String = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("reqwest execution failed");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
