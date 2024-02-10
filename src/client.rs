pub async fn send() {
    let data = r#"
    {
        "test": "hello"
    }"#;
    let data: serde_json::Value = serde_json::from_str(data).unwrap();

    let client = reqwest::Client::new();
    client
        .post("http://localhost:3000/start")
        .json(&data)
        .send()
        .await
        .expect("sending post request failed");
}
