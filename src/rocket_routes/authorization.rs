use rocket::{post, response::status::Custom, serde::json::Json};
use rocket_db_pools::Connection;
use serde_json::{json, Value};

use crate::{
    auth::{authorize_user, Credentials},
    repositories::UserRepository,
};

use super::{server_error, DbConn};

#[post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    mut db: Connection<DbConn>,
    credentials: Json<Credentials>,
) -> Result<Value, Custom<Value>> {
    UserRepository::find_by_username(&mut db, &credentials.username)
        .await
        .map(|user| {
            if let Ok(token) = authorize_user(&user, credentials.into_inner()) {
                return json!(token);
            }
            json!("Unauthorized")
        })
        .map_err(|e| server_error(e.into()))
}
