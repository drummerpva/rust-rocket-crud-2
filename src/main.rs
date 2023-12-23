use rocket_db_pools::Database;
use rocket_routes::{rustaceans::get_rustaceans, DbConn};

mod models;
mod repositories;
mod rocket_routes;
mod schema;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![get_rustaceans])
        .attach(DbConn::init())
        .launch()
        .await;
}
