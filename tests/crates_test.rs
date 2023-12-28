use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

mod common;
use crate::common::{create_crate, create_rustacean, delete_crate, delete_rustacean, URL};

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
fn test_error_on_update_with_long_code() {
    let client = Client::new();
    let rustacean = create_rustacean(&client);
    let crate_data = create_crate(&client);
    let input = json!({
        "rustacean_id": rustacean["id"],
        "code": "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.",
        "name": "test Alt",
        "version": "0.1.1",
        "description": "test Alt"
    });
    let response = client
        .put(URL.to_owned() + "/crates/" + crate_data["id"].to_string().as_str())
        .json(&input)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    delete_crate(&client, crate_data["id"].to_string().as_str());
    delete_rustacean(&client, rustacean["id"].to_string().as_str());
}
#[test]
fn test_error_on_update_with_long_version() {
    let client = Client::new();
    let rustacean = create_rustacean(&client);
    let crate_data = create_crate(&client);
    let input = json!({
        "rustacean_id": rustacean["id"],
        "code": "test alt",
        "name": "test Alt",
        "version": "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.",
        "description": "test Alt"
    });
    let response = client
        .put(URL.to_owned() + "/crates/" + crate_data["id"].to_string().as_str())
        .json(&input)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    delete_crate(&client, crate_data["id"].to_string().as_str());
    delete_rustacean(&client, rustacean["id"].to_string().as_str());
}
#[test]
fn test_error_on_update_with_long_name() {
    let client = Client::new();
    let rustacean = create_rustacean(&client);
    let crate_data = create_crate(&client);
    let input = json!({
        "rustacean_id": rustacean["id"],
        "code": "test alt",
        "name": "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.",
        "version": "1.1.1",
        "description": "test Alt"
    });
    let response = client
        .put(URL.to_owned() + "/crates/" + crate_data["id"].to_string().as_str())
        .json(&input)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    delete_crate(&client, crate_data["id"].to_string().as_str());
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
fn test_error_update_crate_on_inexistent_rustacean() {
    let client = Client::new();
    let crate_data = create_crate(&client);
    let input = json!({
        "rustacean_id": 999999,
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
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
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
