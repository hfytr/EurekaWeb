use actix_web::HttpResponse;
use eureka_web_api::spawn_app;

#[tokio::test]
async fn search_works() {
    let address: String = spawn_app();
    let client = reqwest::Client::new();

    let body = String::from("fen=8%2F8%2FkqQ1K3%2F8%2F8%2F8%2F8%2F8+b+-+-+0+1&wtime=300000&btime=300000&winc=5000&btime=5000"); // fen: 8/8/kqQ1K3/8/8/8/8/8 b - - 0 1 this position has a free capture and bestmove should always be same
    let mut search_finished: bool = false;
    let mut info: Vec<String> = Vec::new();
    let mut bestmove: String = String::new();
    while !search_finished {
        let response: HttpResponse = client::get(format!("{}/make_move", address))
            .header("Content-Type", "applicaions/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("failed to send post request");
        search_finished = response.headers().get("Type").unwrap().as_bytes()[0] != "i" as u8;
        if !search_finished {
            info.push(response.body());
        } else {
            bestmove = response.body().split(" ").nth(1);
        }
    }
    assert!(response.status().is_success());
    assert!(info.len() >= 5); // with allocated time, depth 5 should be easy
    assert_eq!(bestmove, "b6c6");
}
