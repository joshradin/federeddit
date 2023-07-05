//! The user service

use crate::bearer::BearerToken;
use crate::User;
use std::error::Error;
use std::ops::{Deref, DerefMut};
use async_trait::async_trait;

/// Defines the actions of a user service
#[async_trait]
pub trait UserService<U: User> {
    type Authenticated: AuthenticatedUser<U>;
    type AuthError;

    /// Log into the user service.
    async fn log_in(&self, user: &str, pass: &[u8]) -> Result<Self::Authenticated, Self::AuthError>;
}

/// Used to define an authenticated user interaction
pub trait AuthenticatedUser<U: User>: Deref<Target = U> + DerefMut<Target = U> {
    /// The bearer token
    fn bearer(&self) -> &BearerToken;
}

