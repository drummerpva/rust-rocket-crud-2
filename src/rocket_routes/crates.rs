use rocket::{
    delete, get,
    http::Status,
    post, put,
    response::status::{Custom, NoContent},
    serde::json::{json, Json, Value},
};
use rocket_db_pools::Connection;

use crate::{
    models::{NewCrate, User},
    repositories::CrateRepository,
};

use super::{server_error, DbConn};

#[get("/crates")]
pub async fn get_crates(mut db: Connection<DbConn>, _user: User) -> Result<Value, Custom<Value>> {
    CrateRepository::find_multiple(&mut db, 100)
        .await
        .map(|register| json!(register))
        .map_err(|e| server_error(e.into()))
}

#[get("/crates/<id>")]
pub async fn get_crate(
    mut db: Connection<DbConn>,
    id: i32,
    _user: User,
) -> Result<Value, Custom<Value>> {
    CrateRepository::find(&mut db, id)
        .await
        .map(|rustacean| json!(rustacean))
        .map_err(|e| server_error(e.into()))
}

#[post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(
    mut db: Connection<DbConn>,
    new_crate: Json<NewCrate>,
    _user: User,
) -> Result<Custom<Value>, Custom<Value>> {
    CrateRepository::create(&mut db, new_crate.into_inner())
        .await
        .map(|register| Custom(Status::Created, json!(register)))
        .map_err(|e| server_error(e.into()))
}

#[put("/crates/<id>", format = "json", data = "<create>")]
pub async fn update_crate(
    mut db: Connection<DbConn>,
    id: i32,
    create: Json<NewCrate>,
    _user: User,
) -> Result<Value, Custom<Value>> {
    CrateRepository::find(&mut db, id).await.map_err(|_| {
        Custom(
            Status::NotFound,
            json!("Crate with the given id was not found"),
        )
    })?;
    CrateRepository::update(&mut db, id, create.into_inner())
        .await
        .map(|register| json!(register))
        .map_err(|e| server_error(e.into()))
}

#[delete("/crates/<id>")]
pub async fn delete_crate(
    mut db: Connection<DbConn>,
    id: i32,
    _user: User,
) -> Result<NoContent, Custom<Value>> {
    CrateRepository::delete(&mut db, id)
        .await
        .map(|_| NoContent)
        .map_err(|e| server_error(e.into()))
}
