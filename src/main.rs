use rocket_db_pools::Database;
use rocket_routes::{
    rustaceans::{
        create_rustacean, delete_rustacean, get_rustacean, get_rustaceans, update_rustacean,
    },
    DbConn,
};

mod models;
mod repositories;
mod rocket_routes;
mod schema;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![
                get_rustaceans,
                get_rustacean,
                create_rustacean,
                update_rustacean,
                delete_rustacean
            ],
        )
        .attach(DbConn::init())
        .launch()
        .await;
}
