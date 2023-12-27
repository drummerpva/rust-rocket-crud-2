use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{serde_json::json, Value};
const URL: &str = "http://localhost:8000";

fn create_rustacean(client: &Client) -> Value {
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
fn delete_rustacena(client: &Client, id: &str) {
    let response = client
        .delete(URL.to_owned() + "/rustaceans/" + id)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[test]
fn test_create_rustaceans() {
    let client = Client::new();
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
    delete_rustacena(&client, output["id"].to_string().as_str());
}
#[test]
fn test_update_rustaceans() {
    let client = Client::new();
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
    delete_rustacena(&client, output["id"].to_string().as_str());
}

#[test]
fn test_get_rustaceans() {
    let client = Client::new();
    let rustacean1 = create_rustacean(&client);
    let rustacean2 = create_rustacean(&client);
    let response = client.get(URL.to_owned() + "/rustaceans").send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let output: Value = response.json().unwrap();
    assert!(output.as_array().unwrap().len() > 1);
    assert!(output.as_array().unwrap().contains(&rustacean1));
    assert!(output.as_array().unwrap().contains(&rustacean2));
    delete_rustacena(&client, rustacean1["id"].to_string().as_str());
    delete_rustacena(&client, rustacean2["id"].to_string().as_str());
}
#[test]
fn test_get_rustacean() {
    let client = Client::new();
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
    delete_rustacena(&client, output["id"].to_string().as_str());
}
#[test]
fn test_delete_rustacean() {
    let client = Client::new();
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
