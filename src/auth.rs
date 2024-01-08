use argon2::{
    password_hash::{rand_core::Error, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};
use serde::Deserialize;

use crate::models::User;
#[derive(Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub fn authorize_user(user: &User, credentials: Credentials) -> Result<String, Error> {
    let argon = Argon2::default();
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

pub fn hash_password(clean_password: &String) -> String {
    let salt = SaltString::generate(OsRng);
    let argon = Argon2::default();
    let password_hash = argon
        .hash_password(clean_password.as_bytes(), &salt)
        .expect("Error on hash password")
        .to_string();

    return password_hash;
}
