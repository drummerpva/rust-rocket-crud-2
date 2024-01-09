use rocket::{
    delete, get,
    http::Status,
    post, put,
    response::status::{Custom, NoContent},
    serde::json::{json, Json, Value},
};
use rocket_db_pools::Connection;

use crate::{
    models::{NewRustacean, User},
    repositories::RustaceaRepository,
};

use super::{server_error, DbConn};

#[get("/rustaceans")]
pub async fn get_rustaceans(
    mut db: Connection<DbConn>,
    _user: User,
) -> Result<Value, Custom<Value>> {
    RustaceaRepository::find_multiple(&mut db, 100)
        .await
        .map(|rustaceans| json!(rustaceans))
        .map_err(|e| server_error(e.into()))
}

#[get("/rustaceans/<id>")]
pub async fn get_rustacean(
    mut db: Connection<DbConn>,
    id: i32,
    _user: User,
) -> Result<Value, Custom<Value>> {
    RustaceaRepository::find(&mut db, id)
        .await
        .map(|rustacean| json!(rustacean))
        .map_err(|e| server_error(e.into()))
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(
    mut db: Connection<DbConn>,
    new_rustacean: Json<NewRustacean>,
    _user: User,
) -> Result<Custom<Value>, Custom<Value>> {
    RustaceaRepository::create(&mut db, new_rustacean.into_inner())
        .await
        .map(|rustacean| Custom(Status::Created, json!(rustacean)))
        .map_err(|e| server_error(e.into()))
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustacean(
    mut db: Connection<DbConn>,
    id: i32,
    rustacean: Json<NewRustacean>,
    _user: User,
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
        .map_err(|e| server_error(e.into()))
}

#[delete("/rustaceans/<id>")]
pub async fn delete_rustacean(
    mut db: Connection<DbConn>,
    id: i32,
    _user: User,
) -> Result<NoContent, Custom<Value>> {
    RustaceaRepository::delete(&mut db, id)
        .await
        .map(|_| NoContent)
        .map_err(|e| server_error(e.into()))
}
