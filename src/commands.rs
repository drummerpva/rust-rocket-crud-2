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
        let user = UserRepository::create(&mut self.connection, new_user, role_codes)
            .await
            .expect("Error on insertin new user");
        println!("User created {:?}", user);
        let roles = RoleRepository::find_by_user(&mut self.connection, &user)
            .await
            .expect("Error on finding roles by user");
        println!("Roles Assigned {:?}", roles);
    }

    pub async fn list_users(&mut self) {}

    pub async fn delete_user(&mut self, id: i32) {}
}
