use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};
use std::process::Command;

use crate::common::URL;
mod common;

#[test]
fn test_login_ensure_return_session_id() {
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
    assert_eq!(response.status(), StatusCode::OK);
    let output: Value = response.json().unwrap();
    assert!(output.get("token").is_some());
    assert!(output["token"].is_string());
    assert_eq!(output["token"].as_str().unwrap().len(), 128);
}
#[test]
fn test_login_ensure_return_unauthorized_on_invalid_password() {
    let client = Client::new();
    let input = json!({
        "username": "test_admin",
        "password": "test_invalid"
    });
    let response = client
        .post(URL.to_owned() + "/login")
        .json(&input)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
