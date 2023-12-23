use rocket::get;
use rocket_db_pools::{Connection, Database};

mod models;
mod repositories;
mod schema;

#[derive(Database)]
#[database("postgres")]
struct DbConn(rocket_db_pools::diesel::PgPool);

#[get("/rustaceans")]
fn get_rustaceans(db: Connection<DbConn>) {}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![get_rustaceans])
        .attach(DbConn::init())
        .launch()
        .await;
}
