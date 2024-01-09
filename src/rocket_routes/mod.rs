use std::error::Error;

use rocket::{
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest},
    response::status::Custom,
    Request,
};
use rocket_db_pools::{deadpool_redis::redis::AsyncCommands, Connection, Database};
use serde_json::{json, Value};

use crate::{models::User, repositories::UserRepository};

pub mod authorization;
pub mod crates;
pub mod rustaceans;

#[derive(Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

#[derive(Database)]
#[database("redis")]
pub struct CacheConn(rocket_db_pools::deadpool_redis::Pool);

pub fn server_error(e: Box<dyn Error>) -> Custom<Value> {
    rocket::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // Authorization: Beaerr SESSION_ID_128_CHARS_LONG
        let session_header = req
            .headers()
            .get_one("Authorization")
            .map(|value| value.split_whitespace().collect::<Vec<_>>())
            .filter(|value| value.len() == 2 && value[0] == "Bearer");
        if let Some(header_value) = session_header {
            let mut cache = req
                .guard::<Connection<CacheConn>>()
                .await
                .expect("Can not connect to Redis in request guard");
            let mut db_connection = req
                .guard::<Connection<DbConn>>()
                .await
                .expect("Can not connect to Database in request guard");
            let cache_data = cache
                .get::<String, i32>(format!("sessions/{}", header_value[1]))
                .await;
            if let Ok(user_id) = cache_data {
                if let Ok(user) = UserRepository::find(&mut db_connection, user_id).await {
                    return Outcome::Success(user);
                }
            }
        }
        Outcome::Error((Status::Unauthorized, ()))
    }
}
