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
        println!("User created {user:?}");
        let roles = RoleRepository::find_by_user(&mut self.connection, &user)
            .await
            .expect("Error on finding roles by user");
        println!("Roles Assigned {roles:?}");
    }

    pub async fn list_users(&mut self) {
        let users = UserRepository::find_with_roles(&mut self.connection)
            .await
            .expect("Error on get users with roles");
        for user in users {
            println!("User: {user:?}");
        }
    }

    pub async fn delete_user(&mut self, id: i32) {
        UserRepository::delete(&mut self.connection, id)
            .await
            .expect("Error on delete user");
        println!("User deleted");
    }
}
