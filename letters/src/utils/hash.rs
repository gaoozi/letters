use anyhow::anyhow;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use base64::{engine::general_purpose, Engine as _};

pub fn generate_hash(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| anyhow!("Failed to verify password"))?;
    Ok(general_purpose::STANDARD_NO_PAD.encode(password_hash.to_string()))
}

pub fn verify_password(password: &str, password_hash: &str) -> anyhow::Result<bool> {
    let decode_hash = general_purpose::STANDARD_NO_PAD
        .decode(password_hash)
        .unwrap();
    let decode_hash_str = String::from_utf8_lossy(&decode_hash);

    let password_hash = PasswordHash::new(&decode_hash_str)?;
    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok())
}
