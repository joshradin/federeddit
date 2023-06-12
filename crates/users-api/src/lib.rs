//! User apis


use chrono::{DateTime, Utc};
pub use email_address::EmailAddress;

pub mod user_service;
pub mod bearer;
pub mod error;
pub mod header;
pub mod guard;
pub mod auth;

/// Common type for expiration
pub type ExpirationTime = DateTime<Utc>;

/// The users trait
pub trait User {
    /// Gets the username of the user
    fn username(&self) -> &str;
    /// Sets the username of this user
    fn set_username(&mut self, name: &str);

    /// Gets the email of the user
    fn email(&self) -> EmailAddress;
}
