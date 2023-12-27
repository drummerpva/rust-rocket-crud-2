use rocket::{
    delete, get,
    http::Status,
    post, put,
    response::status::{Custom, NoContent},
    serde::json::{json, Json, Value},
};
use rocket_db_pools::Connection;

use crate::{models::NewCrate, repositories::CrateRepository};

use super::DbConn;

#[get("/crates")]
pub async fn get_crates(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    CrateRepository::find_multiple(&mut db, 100)
        .await
        .map(|register| json!(register))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[get("/crates/<id>")]
pub async fn get_crate(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
    CrateRepository::find(&mut db, id)
        .await
        .map(|rustacean| json!(rustacean))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(
    mut db: Connection<DbConn>,
    new_crate: Json<NewCrate>,
) -> Result<Custom<Value>, Custom<Value>> {
    CrateRepository::create(&mut db, new_crate.into_inner())
        .await
        .map(|register| Custom(Status::Created, json!(register)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[put("/crates/<id>", format = "json", data = "<create>")]
pub async fn update_crate(
    mut db: Connection<DbConn>,
    id: i32,
    create: Json<NewCrate>,
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
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[delete("/crates/<id>")]
pub async fn delete_crate(mut db: Connection<DbConn>, id: i32) -> Result<NoContent, Custom<Value>> {
    CrateRepository::delete(&mut db, id)
        .await
        .map(|_| NoContent)
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}
