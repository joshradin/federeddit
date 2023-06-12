//! The user service

use std::error::Error;
use std::ops::{Deref, DerefMut};
use crate::bearer::BearerToken;
use crate::User;

/// Defines the actions of a user service
pub trait UserService<U : User> {
    type Authenticated: AuthenticatedUser<U>;
    type AuthError: Error;

    /// Log into the user service.
    fn log_in(&self, user: &str,  pass: &[u8]) -> Result<Self::Authenticated, Self::AuthError>;
}

/// Used to define an authenticated user interaction
pub trait AuthenticatedUser<U : User> : Deref<Target=U> + DerefMut<Target=U>{

    /// The bearer token
    fn bearer(&self) -> &BearerToken;
}