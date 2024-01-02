use std::env;

use diesel_async::{AsyncConnection, AsyncPgConnection};

pub async fn load_db_connection() -> AsyncPgConnection {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    AsyncPgConnection::establish(&database_url)
        .await
        .expect("Cannon connect to postgres")
}
