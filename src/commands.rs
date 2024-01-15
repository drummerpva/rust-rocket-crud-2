use std::{env, str::FromStr};

use chrono::{Datelike, Utc};
use diesel_async::AsyncPgConnection;
use lettre::{
    message::{header::ContentType, MessageBuilder},
    transport::smtp::authentication::Credentials,
    SmtpTransport, Transport,
};
use tera::{Context, Tera};

use crate::{
    auth::hash_password,
    models::{NewUser, RoleCode},
    repositories::{CrateRepository, RoleRepository, UserRepository},
};

pub struct CommandsServices {
    connection: AsyncPgConnection,
}

fn load_template_engine() -> Tera {
    Tera::new("templates/**/*.html").expect("Cannoit load template engine")
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
        let new_user = NewUser {
            username,
            password: hash_password(&password),
        };
        let role_enums = role_codes
            .iter()
            .map(|role_code| RoleCode::from_str(role_code.as_str()).unwrap())
            .collect();
        let user = UserRepository::create(&mut self.connection, new_user, role_enums)
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

    pub async fn digest_send(&mut self, email: String, hours_since: i32) {
        let crates = CrateRepository::find_since(&mut self.connection, hours_since)
            .await
            .unwrap();
        if crates.len() > 0 {
            let tera = load_template_engine();
            let mut context = Context::new();
            let year = Utc::now().year();
            context.insert("year", &year);
            context.insert("crates", &crates);

            let html_body = tera
                .render("email/digest.html", &context)
                .expect("Error on generate digest html");
            let message = MessageBuilder::new()
                .subject("Cr8s Digest")
                .from("Cr8s <noreply@cr8s.com>".parse().unwrap())
                .to(email.parse().unwrap())
                .header(ContentType::TEXT_HTML)
                .body(html_body)
                .expect("Error on crate message to email with MessageBuidler");

            dotenv::dotenv().ok();
            let smtp_host = env::var("SMTP_HOST").expect("SMTP_HOST must be set");
            let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
            let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
            let credentials = Credentials::new(smtp_username, smtp_password);
            let mailer = SmtpTransport::relay(&smtp_host)
                .expect("Error on create SMTP Transport")
                .credentials(credentials)
                .build();
            mailer.send(&message).expect("Error on send email");
        }
    }
}
