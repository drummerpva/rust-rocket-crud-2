use argon2::{password_hash::rand_core::Error, PasswordHash, PasswordVerifier};
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;

use crate::models::User;
#[derive(Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub fn authorize_user(user: &User, credentials: Credentials) -> Result<String, Error> {
    let argon = argon2::Argon2::default();
    let db_hash = PasswordHash::new(&user.password).unwrap();
    argon
        .verify_password(credentials.password.as_bytes(), &db_hash)
        .unwrap();
    let session_id = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect();
    Ok(session_id)
}
