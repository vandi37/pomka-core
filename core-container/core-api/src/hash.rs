// src/password.rs
use argon2::{
    Algorithm, Argon2, Params, Version,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use rand_core::OsRng;

#[derive(Clone)]
pub struct PasswordHasherService {
    argon2: Argon2<'static>,
}

impl PasswordHasherService {
    pub fn new() -> Result<Self, argon2::password_hash::Error> {
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13,  Params::new(
            128 * 1024,
            3,
            4,
            None,
        )?);

        Ok(Self {
                    argon2,
                })
    }

    pub fn hash_password(&self, password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);

        self.argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|ph| ph.to_string())
    }

    pub fn verify_password(
        &self,
        hash: &str,
        password: &str,
    ) -> Result<bool, argon2::password_hash::Error> {
        let parsed_hash = PasswordHash::new(hash)?;

        Ok(self
            .argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}
