use eureka_web_api::spawn_app;
use log::debug;

#[tokio::test]
async fn search_works() {
    let address: String = spawn_app();
    let client = reqwest::Client::new();
    let request_body = String::from("fen=8%2F8%2FkqQ1K3%2F8%2F8%2F8%2F8%2F8+b+-+-+0+1&wtime=300000&btime=300000&winc=5000&binc=5000"); // fen: 8/8/kqQ1K3/8/8/8/8/8 b - - 0 1 this position has a free capture and bestmove should always be the same
    let response = client
        .get(format!("{}/search", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(request_body)
        .send()
        .await
        .expect("failed to send post request");
    debug!("{:?}", response);
    let body = response.text().await.unwrap();
    debug!("{}", body);

    let body: Vec<&str> = body.split("\n").collect(); // necessary to iterator has sometething to point to
    let best_move: &str = body[2].split(" ").nth(1).unwrap();
    assert_eq!(best_move, "b6c6");
}