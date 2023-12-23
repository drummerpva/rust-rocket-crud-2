use rocket::{
    get,
    http::Status,
    response::status::Custom,
    serde::json::{json, Value},
};
use rocket_db_pools::Connection;

use crate::repositories::RustaceaRepository;

use super::DbConn;

#[get("/rustaceans")]
pub async fn get_rustaceans(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    RustaceaRepository::find_multiple(&mut db, 100)
        .await
        .map(|rustaceans| json!(rustaceans))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}
