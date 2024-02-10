use axum::routing::{get, post};
use axum::{Json, Router};
use command::Command;
use std::io;

mod client;
mod command;

#[tokio::main]
async fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("the input should be a string");

        execute_command(input.trim()).await;
    }
}

async fn execute_command(command: &str) {
    let test = Command::from_str(command).unwrap();

    match command {
        "start" => client::send().await,
        "end" => client::send().await,
        "host" => host().await,
        "test" => println!("{}", test.name),
        _ => (),
    }
}

async fn test(Json(payload): Json<serde_json::Value>) {
    println!("post request received");
    println!("{}", payload);
}

async fn host() {
    let app = Router::new().route("/start", post(test));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("server started");
    axum::serve(listener, app).await.unwrap();
}
