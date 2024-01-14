use actix_web::{web, HttpResponse, Responder};
use log::debug;
use serde::Deserialize;
use std::process::Stdio;
use std::sync::Mutex;
use tokio::{io::AsyncBufReadExt, process};

#[derive(Deserialize)]
pub struct SearchRequest {
    fen: String,
    wtime: u32,
    btime: u32,
    winc: u32,
    binc: u32,
}

pub struct EngineCommunicator {
    engine_in: process::ChildStdin,
    engine_out: tokio::io::BufReader<process::ChildStdout>,
    currently_searching: bool,
}

impl EngineCommunicator {
    pub fn new() -> EngineCommunicator {
        let mut handle = process::Command::new("engine/EurekaUCI")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let handle_in = handle.stdin.take().unwrap();
        let handle_out = tokio::io::BufReader::new(handle.stdout.take().unwrap());

        EngineCommunicator {
            engine_in: handle_in,
            engine_out: handle_out,
            currently_searching: false,
        }
    }

    pub async fn start_search(
        request: web::Form<SearchRequest>,
        engine_handle: web::Data<Mutex<EngineCommunicator>>,
    ) -> impl Responder {
        println!("hi from start_search");
        let mut engine_handle = engine_handle.lock().unwrap();
        engine_handle.currently_searching = true;
        let go_cmd = format!(
            "position fen {}\ngo wtime {} btime {} winc {} binc {}\n",
            request.fen, request.wtime, request.btime, request.winc, request.binc
        );

        use tokio::io::AsyncWriteExt;
        engine_handle
            .engine_in
            .write_all(go_cmd.as_ref())
            .await
            .unwrap();
        engine_handle.engine_in.flush().await.unwrap();

        HttpResponse::Ok()
    }

    pub async fn get_info(engine_handle: web::Data<Mutex<EngineCommunicator>>) -> impl Responder {
        println!("hi from get_info");
        let mut engine_handle = engine_handle.lock().unwrap();

        if !engine_handle.currently_searching {
            return HttpResponse::BadRequest()
                .body("no search started, start a search with post request to /search/start");
        }

        let mut body: String = String::new();
        engine_handle.engine_out.read_line(&mut body).await.unwrap();
        let response_type: &str = if body.as_bytes()[0] == 'i' as u8 {
            "info"
        } else {
            "move"
        };

        // in case of info, need to read extra line for pv
        if response_type == "info" {
            let mut pv: String = String::new();
            engine_handle.engine_out.read_line(&mut pv).await.unwrap();
            body.push_str(&pv);
        } else {
            engine_handle.currently_searching = false;
        }

        HttpResponse::Ok()
            .insert_header(("Type", response_type))
            .body(body)
    }
}
