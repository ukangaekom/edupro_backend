use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, PasswordHash};
use rand::rngs::OsRng;





pub async fn encrypt_password(password: &str) -> String {

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2.hash_password(password.as_bytes(),&salt).unwrap().to_string()

}

pub async fn verify_password(hash: &str, password: &str) -> bool {
    let parsed = PasswordHash::new(hash).unwrap();
    Argon2::default().verify_password(password.as_bytes(),&parsed).is_ok()
}