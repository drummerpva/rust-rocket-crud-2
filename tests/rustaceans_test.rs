use reqwest::StatusCode;
use serde_json::{json, Value};

mod common;
use crate::common::{
    create_rustacean, delete_rustacean, get_client_with_logged_in_admin,
    get_client_with_logged_in_viewer, URL,
};

#[test]
fn test_create_rustaceans() {
    let client = get_client_with_logged_in_admin();
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
    assert_eq!(output["name"], input["name"]);
    assert_eq!(output["email"], input["email"]);
    assert!(output["id"].is_number());
    assert!(output["created_at"].is_string());
    delete_rustacean(&client, output["id"].to_string().as_str());
}

#[test]
fn test_create_rustaceans_viewer() {
    let client = get_client_with_logged_in_viewer();
    let input = json!({
        "name": "John Doe",
        "email": "john@doe.com"
    });
    let response = client
        .post(URL.to_owned() + "/rustaceans")
        .json(&input)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
#[test]
fn test_update_rustaceans() {
    let client = get_client_with_logged_in_admin();
    let output_create = create_rustacean(&client);
    let input_update = json!({
        "name": "Jane Doe Alt",
        "email": "altJohn@doe.com"
    });
    let response_update = client
        .put(URL.to_owned() + "/rustaceans/" + output_create["id"].to_string().as_str())
        .json(&input_update)
        .send()
        .unwrap();
    assert_eq!(response_update.status(), StatusCode::OK);
    let output: Value = response_update.json().unwrap();
    assert_eq!(output["name"], input_update["name"]);
    assert_eq!(output["email"], input_update["email"]);
    assert_eq!(output_create["id"], output["id"]);
    assert_eq!(output_create["created_at"], output["created_at"]);
    delete_rustacean(&client, output["id"].to_string().as_str());
}
#[test]
fn test_update_rustaceans_viewer() {
    let client_admin = get_client_with_logged_in_admin();
    let output_create = create_rustacean(&client_admin);
    let input_update = json!({
        "name": "Jane Doe Alt",
        "email": "altJohn@doe.com"
    });
    let client_viewer = get_client_with_logged_in_viewer();
    let response_update = client_viewer
        .put(URL.to_owned() + "/rustaceans/" + output_create["id"].to_string().as_str())
        .json(&input_update)
        .send()
        .unwrap();
    assert_eq!(response_update.status(), StatusCode::UNAUTHORIZED);
}
#[test]
fn test_get_rustaceans() {
    let client = get_client_with_logged_in_admin();
    let rustacean1 = create_rustacean(&client);
    let rustacean2 = create_rustacean(&client);
    let response = client.get(URL.to_owned() + "/rustaceans").send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let output: Value = response.json().unwrap();
    assert!(output.as_array().unwrap().len() > 1);
    delete_rustacean(&client, rustacean1["id"].to_string().as_str());
    delete_rustacean(&client, rustacean2["id"].to_string().as_str());
}
#[test]
fn test_get_rustacean() {
    let client = get_client_with_logged_in_admin();
    let create_data: Value = create_rustacean(&client);
    let response = client
        .get(URL.to_owned() + "/rustaceans/" + create_data["id"].to_string().as_str())
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let output: Value = response.json().unwrap();
    assert_eq!(output["name"], create_data["name"]);
    assert_eq!(output["email"], create_data["email"]);
    assert!(output["id"].is_number());
    assert!(output["created_at"].is_string());
    assert_eq!(output["id"], create_data["id"]);
    delete_rustacean(&client, output["id"].to_string().as_str());
}
#[test]
fn test_delete_rustacean() {
    let client = get_client_with_logged_in_admin();
    let create_data: Value = create_rustacean(&client);
    let response = client
        .delete(URL.to_owned() + "/rustaceans/" + create_data["id"].to_string().as_str())
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    let get_response = client
        .get(URL.to_owned() + "/rustaceans/" + create_data["id"].to_string().as_str())
        .send()
        .unwrap();
    assert_eq!(get_response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
#[test]
fn test_delete_rustacean_viewer() {
    let client_admin = get_client_with_logged_in_admin();
    let create_data: Value = create_rustacean(&client_admin);
    let client_viewer = get_client_with_logged_in_viewer();
    let response = client_viewer
        .delete(URL.to_owned() + "/rustaceans/" + create_data["id"].to_string().as_str())
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
