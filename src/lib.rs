use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, PasswordHash,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|error| error.to_string())
}

#[wasm_bindgen]
pub fn verify_password(password: &str, hash: &str) -> bool {
    if let Ok(hash) = PasswordHash::new(hash) {
        Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .is_ok()
    } else {
        false
    }
}
