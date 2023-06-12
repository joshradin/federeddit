//! Defines the auth service

use crate::bearer::BearerToken;
use crate::error::AuthError;
use crate::ExpirationTime;
use argon2::password_hash::{Salt, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use std::time::Instant;
use thiserror::Error;

/// Used for confirming if a bearer token is valid
pub trait AuthService {
    /// Validates a bearer token, returning whether it's valid or not. If valid, the expiration time is
    /// returned. Otherwise, an auth error is returned.
    fn validate_token(&self, token: &BearerToken) -> Result<ExpirationTime, AuthError>;
}

/// The password hash factory
#[derive(Debug, Clone)]
pub struct PasswordAuth {
    salt: SaltString,
}

impl PasswordAuth {
    /// Creates a new password authorizer using a b64 encoded secret
    pub fn new(salt: &str) -> Self {
        Self {
            salt: SaltString::from_b64(salt).expect("given salt is not in b64 format"),
        }
    }

    /// Hashes a password
    pub fn hash_password(&self, password: &[u8]) -> Result<String, PasswordError> {
        let argon = Argon2::default();
        let hashed = argon
            .hash_password(password, &self.salt)
            .map_err(|e| PasswordError::InvalidPasswordHash(e.to_string()))?;
        Ok(hashed.to_string())
    }

    /// Verifies a password against a hash
    pub fn verify_password(&self, password: &[u8], hash: &str) -> Result<(), PasswordError> {
        let ref parsed = PasswordHash::new(hash)
            .map_err(|e| PasswordError::InvalidPasswordHash(e.to_string()))?;
        Argon2::default()
            .verify_password(password, parsed)
            .map_err(|e| PasswordError::IncorrectPassword)
    }
}

impl From<SaltString> for PasswordAuth {
    fn from(value: SaltString) -> Self {
        Self {
            salt: value
        }
    }
}


/// An error occurred with passwords
#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("given password is wrong")]
    IncorrectPassword,
    #[error("invalid password hash: {0}")]
    InvalidPasswordHash(String),
    #[error("no password was found")]
    NoPasswordFound
}
