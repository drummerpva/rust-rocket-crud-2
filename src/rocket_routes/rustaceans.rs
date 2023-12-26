use rocket::{
    delete, get,
    http::Status,
    post, put,
    response::status::{Custom, NoContent},
    serde::json::{json, Json, Value},
};
use rocket_db_pools::Connection;

use crate::{models::NewRustacean, repositories::RustaceaRepository};

use super::DbConn;

#[get("/rustaceans")]
pub async fn get_rustaceans(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    RustaceaRepository::find_multiple(&mut db, 100)
        .await
        .map(|rustaceans| json!(rustaceans))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[get("/rustaceans/<id>")]
pub async fn get_rustacean(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
    RustaceaRepository::find(&mut db, id)
        .await
        .map(|rustacean| json!(rustacean))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(
    mut db: Connection<DbConn>,
    new_rustacean: Json<NewRustacean>,
) -> Result<Value, Custom<Value>> {
    RustaceaRepository::create(&mut db, new_rustacean.into_inner())
        .await
        .map(|rustacean| json!(rustacean))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustacean(
    mut db: Connection<DbConn>,
    id: i32,
    rustacean: Json<NewRustacean>,
) -> Result<Value, Custom<Value>> {
    RustaceaRepository::find(&mut db, id).await.map_err(|_| {
        Custom(
            Status::NotFound,
            json!("Rustacean with the given id was not found"),
        )
    })?;
    RustaceaRepository::update(&mut db, id, rustacean.into_inner())
        .await
        .map(|rustacean| json!(rustacean))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[delete("/rustaceans/<id>")]
pub async fn delete_rustacean(
    mut db: Connection<DbConn>,
    id: i32,
) -> Result<NoContent, Custom<Value>> {
    RustaceaRepository::delete(&mut db, id)
        .await
        .map(|_| NoContent)
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}
