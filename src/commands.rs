use diesel_async::AsyncPgConnection;

use crate::{
    models::NewUser,
    repositories::{RoleRepository, UserRepository},
};

pub struct CommandsServices {
    connection: AsyncPgConnection,
}

impl CommandsServices {
    pub async fn new(connection: AsyncPgConnection) -> Self {
        Self { connection }
    }
    pub async fn create_user(
        &mut self,
        username: String,
        password: String,
        role_codes: Vec<String>,
    ) {
        let new_user = NewUser { username, password };
        let user = UserRepository::create(&mut self.connection, new_user)
            .await
            .expect("Error on insertin new user");
        println!("User created {:?}", user);
    }

    pub async fn list_users(&mut self) {}

    pub async fn delete_user(&mut self, id: i32) {}
}
