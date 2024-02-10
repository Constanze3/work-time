use axum::routing::post;
use axum::{Json, Router};
use client::{Client, LocalClient, NetworkedClient, WorkData};
use reqwest::Url;
use std::io;
use std::path::Path;

mod client;
mod command;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("the input should be a string");

        execute_command(input.trim());
    }
}

fn execute_command(command: &str) {
    let dir = std::env::current_dir().unwrap();
    let local_client = LocalClient::from_config(dir.as_path()).unwrap();

    let url = Url::parse("http://localhost:3000").unwrap();
    let client = NetworkedClient::new(local_client, url);

    match command {
        "start" => client.work_started(),
        "end" => client.work_ended(WorkData {
            description: String::from("did nothing"),
            category: String::from("lazing"),
        }),
        "host" => host(),
        _ => (),
    }
}

async fn test(Json(payload): Json<serde_json::Value>) {
    println!("post request received");
    println!("{}", payload);
}

fn host() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let app = Router::new()
            .route("/start", post(test))
            .route("/end", post(test));
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

        println!("server started");
        axum::serve(listener, app).await.unwrap();
    })
}
