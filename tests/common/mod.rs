use std::process::Command;

use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{serde_json::json, Value};
pub const URL: &str = "http://localhost:8000";
pub fn create_rustacean(client: &Client) -> Value {
    let input = json!({
        "name": "John Doe",
        "email": "john@doe.com"
    });
    let response = client
        .post(URL.to_owned() + "/rustaceans")
        .json(&input)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let output: Value = response.json().unwrap();
    output
}
pub fn create_crate(client: &Client) -> Value {
    let rustacean = create_rustacean(&client);
    let input = json!({
        "rustacean_id": rustacean["id"],
        "code": "test",
        "name": "test",
        "version": "0.1.0",
        "description": "test"
    });
    let response = client
        .post(URL.to_owned() + "/crates")
        .json(&input)
        .send()
        .unwrap();
    let output: Value = response.json().unwrap();
    output
}
pub fn delete_rustacean(client: &Client, id: &str) {
    let response = client
        .delete(URL.to_owned() + "/rustaceans/" + id)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
pub fn delete_crate(client: &Client, id: &str) {
    let response = client
        .delete(URL.to_owned() + "/crates/" + id)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

pub fn get_client_with_logged_in_admin() -> Client {
    let _ = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg("test_admin")
        .arg("test")
        .arg("admin")
        .output()
        .unwrap();
    let client = Client::new();
    let input = json!({
        "username": "test_admin",
        "password": "test"
    });
    let response = client
        .post(URL.to_owned() + "/login")
        .json(&input)
        .send()
        .unwrap();
    let output: Value = response.json().unwrap();
    assert!(output.get("token").is_some());
    let mut headers = reqwest::header::HeaderMap::new();
    let token = output["token"].as_str().unwrap();
    let authorization_value = format!("Bearer {}", token);
    headers.insert(
        reqwest::header::AUTHORIZATION,
        reqwest::header::HeaderValue::from_str(authorization_value.as_str()).unwrap(),
    );
    Client::builder().default_headers(headers).build().unwrap()
}
