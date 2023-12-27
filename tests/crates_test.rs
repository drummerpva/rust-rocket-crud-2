use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{serde_json::json, Value};

use crate::common::{create_crate, create_rustacean, delete_crate, delete_rustacean, URL};
mod common;

#[test]
fn test_create_crate() {
    let client = Client::new();
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
    assert_eq!(response.status(), StatusCode::CREATED);
    let output: Value = response.json().unwrap();
    assert_eq!(output["rustacean_id"], input["rustacean_id"]);
    assert_eq!(output["code"], input["code"]);
    assert_eq!(output["name"], input["name"]);
    assert_eq!(output["version"], input["version"]);
    assert_eq!(output["description"], input["description"]);
    assert!(output["id"].is_number());
    assert!(output["created_at"].is_string());
    delete_crate(&client, output["id"].to_string().as_str());
    delete_rustacean(&client, rustacean["id"].to_string().as_str());
}
#[test]
fn test_update_crate() {
    let client = Client::new();
    let rustacean = create_rustacean(&client);
    let crate_data = create_crate(&client);
    let input = json!({
        "rustacean_id": rustacean["id"],
        "code": "test Alt",
        "name": "test Alt",
        "version": "0.1.1",
        "description": "test Alt"
    });
    let response = client
        .put(URL.to_owned() + "/crates/" + crate_data["id"].to_string().as_str())
        .json(&input)
        .send()
        .unwrap();
    let output: Value = response.json().unwrap();
    assert_eq!(output["rustacean_id"], input["rustacean_id"]);
    assert_eq!(output["code"], input["code"]);
    assert_eq!(output["name"], input["name"]);
    assert_eq!(output["version"], input["version"]);
    assert_eq!(output["description"], input["description"]);
    assert!(output["id"].is_number());
    assert!(output["created_at"].is_string());
    assert_eq!(output["id"], crate_data["id"]);
    assert_eq!(output["created_at"], crate_data["created_at"]);
    delete_crate(&client, output["id"].to_string().as_str());
    delete_rustacean(&client, rustacean["id"].to_string().as_str());
}
#[test]
fn test_view_crates() {
    let client = Client::new();
    let crate_data = create_crate(&client);
    let response = client.get(URL.to_owned() + "/crates").send().unwrap();
    let output: Value = response.json().unwrap();
    assert!(output.as_array().unwrap().len() > 0);
    assert!(output.as_array().unwrap().contains(&crate_data));
    delete_crate(&client, crate_data["id"].to_string().as_str());
    delete_rustacean(&client, crate_data["rustacean_id"].to_string().as_str());
}
#[test]
fn test_get_crate() {
    let client = Client::new();
    let crate_data = create_crate(&client);
    let response = client
        .get(URL.to_owned() + "/crates/" + crate_data["id"].to_string().as_str())
        .send()
        .unwrap();
    let output: Value = response.json().unwrap();
    assert_eq!(output["rustacean_id"], crate_data["rustacean_id"]);
    assert_eq!(output["code"], crate_data["code"]);
    assert_eq!(output["name"], crate_data["name"]);
    assert_eq!(output["version"], crate_data["version"]);
    assert_eq!(output["description"], crate_data["description"]);
    assert_eq!(output["id"], crate_data["id"]);
    assert_eq!(output["created_at"], crate_data["created_at"]);
    delete_crate(&client, crate_data["id"].to_string().as_str());
    delete_rustacean(&client, crate_data["rustacean_id"].to_string().as_str());
}
#[test]
fn test_delete_crate() {
    let client = Client::new();
    let crate_data = create_crate(&client);
    let response = client
        .delete(URL.to_owned() + "/crates/" + crate_data["id"].to_string().as_str())
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    let get_response = client
        .get(URL.to_owned() + "/crates/" + crate_data["id"].to_string().as_str())
        .send()
        .unwrap();
    assert_eq!(get_response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    delete_rustacean(&client, crate_data["rustacean_id"].to_string().as_str());
}
