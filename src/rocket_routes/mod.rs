use std::error::Error;

use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest},
    response::status::Custom,
    Request, Response,
};
use rocket_db_pools::{deadpool_redis::redis::AsyncCommands, Connection, Database};
use serde_json::{json, Value};

use crate::{
    models::{RoleCode, User},
    repositories::{RoleRepository, UserRepository},
};

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

#[rocket::options("/<_route_args..>")]
pub fn options(_route_args: Option<std::path::PathBuf>) {
    // Just to add ORS header via the fairing
}

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Append CORS headers in responses",
            kind: Kind::Response,
        }
    }
    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_raw_header("Access-Control-Allow-Origin", "*");
        res.set_raw_header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE");
        res.set_raw_header("Access-Control-Allow-Headers", "*");
        res.set_raw_header("Access-Control-Allow-Credentials", "true");
    }
}
pub struct EditorUser(User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for EditorUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let user = req
            .guard::<User>()
            .await
            .expect("Cannont retrieve current logged in user");
        let mut db_connection = req
            .guard::<Connection<DbConn>>()
            .await
            .expect("Can not connect to Database in request guard");
        if let Ok(roles) = RoleRepository::find_by_user(&mut db_connection, &user).await {
            let is_editor = roles.iter().any(|role| match role.code {
                RoleCode::Admin | RoleCode::Editor => true,
                _ => false,
            });
            if is_editor {
                return Outcome::Success(EditorUser(user));
            }
        };
        Outcome::Error((Status::Unauthorized, ()))
    }
}
