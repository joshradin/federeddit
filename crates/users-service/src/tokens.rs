//! Used to define the JWT for bearer auth

use chrono::Duration;
use hmac::digest::KeyInit;
use hmac::Hmac;
use serde::{Deserialize, Serialize};
use sha2::Sha384;
use std::ops::Add;
use std::time::{Instant, SystemTime};
use users_api::bearer::BearerToken;
use users_api::{ExpirationTime, User};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticatedUserToken {
    email: String,
    expiration_time: ExpirationTime,
}

impl AuthenticatedUserToken {
    /// Creates a new authenticated user token with an expiration time
    pub fn new(email: &dyn User, expiration_time: ExpirationTime) -> Self {
        Self {
            email: email.email().to_string(),
            expiration_time,
        }
    }

    /// Creates a new authenticated user token that expires after a set time.
    pub fn with_valid_duration(email: &dyn User, expires_after: Duration) -> Self {
        Self {
            email: email.email().to_string(),
            expiration_time: ExpirationTime::from(SystemTime::now()) + expires_after,
        }
    }

    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn expiration_time(&self) -> ExpirationTime {
        self.expiration_time
    }
}
