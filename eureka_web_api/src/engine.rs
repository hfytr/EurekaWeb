use actix_web::{web, HttpResponse, Responder};
use log::debug;
use serde::Deserialize;
use std::process::Stdio;
use tokio::{io::AsyncBufReadExt, process};

#[derive(Deserialize)]
pub struct SearchRequest {
    fen: String,
    wtime: u32,
    btime: u32,
    winc: u32,
    binc: u32,
}

pub async fn search(request: web::Form<SearchRequest>) -> impl Responder {
    debug!("search called");
    use tokio::io::AsyncWriteExt;
    let mut engine_handle = process::Command::new("engine/EurekaUCI")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut engine_in = engine_handle.stdin.take().unwrap();
    let mut engine_out = tokio::io::BufReader::new(engine_handle.stdout.take().unwrap());

    let go_cmd = format!(
        "position fen {}\ngo wtime {} btime {} winc {} binc {}\n",
        request.fen, request.wtime, request.btime, request.winc, request.binc
    );
    debug!("started search");
    engine_in.write_all(go_cmd.as_ref()).await.unwrap();
    engine_in.flush().await.unwrap();

    let mut input_type: String = String::from("info");
    let mut output: Vec<String> = vec![String::new(), String::new()];
    let mut i = 0;
    let mut line = String::new();
    while input_type == "info" {
        line = String::new();
        i = 1 - i;
        engine_out.read_line(&mut line).await.unwrap();
        input_type = line.split(" ").nth(0).unwrap().to_string();
        if input_type == "info" {
            output[i] = line.clone();
        }
        debug!("info recieved: {}input_type: {}", output[i], input_type);
    }
    let body = format!("{}{}{}", output[0], output[1], line);

    engine_handle
        .kill()
        .await
        .expect("CRITICAL: failed to kill engine process"); // failure can crash terminal

    debug!("{}", body);
    HttpResponse::Ok().body(body)
}
