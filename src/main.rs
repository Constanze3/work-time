use axum::routing::post;
use axum::{Json, Router};
use client::{Client, LocalClient, NetworkedClient, WorkData};
use config::{create_default_config, default_config_path, Config};
use reqwest::Url;
use std::io;

mod client;
mod config;

fn main() {
    let config_path = default_config_path().expect("path to config file should be valid");
    let config = if config_path.exists() {
        Config::load(config_path)
    } else {
        create_default_config(config_path)
    }
    .expect("config file should be valid");

    let local_client = LocalClient::new(config);

    let url = Url::parse("http://localhost:3000").unwrap();
    let test_client = NetworkedClient::new(local_client, url);

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("the input should be a string");

        execute_command(&test_client, input.trim());
    }
}

fn execute_command(client: &impl Client, command: &str) {
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
