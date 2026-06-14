//! Argon2id password hashing and verification.
//!
//! Uses default Argon2 parameters (Argon2id, m=19456, t=2, p=1 in 0.5).
//! Each hash is salted with a fresh `SaltString` via `OsRng`.

use argon2::password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

pub fn verify_password(hash: &str, password: &str) -> argon2::password_hash::Result<()> {
    let parsed = PasswordHash::new(hash)?;
    Argon2::default().verify_password(password.as_bytes(), &parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_then_verify_ok() {
        let h = hash_password("hunter2").unwrap();
        assert!(verify_password(&h, "hunter2").is_ok());
    }

    #[test]
    fn verify_rejects_wrong_password() {
        let h = hash_password("hunter2").unwrap();
        assert!(verify_password(&h, "wrong").is_err());
    }
}
