use crate::auth::PasswordError;
use actix_web::http::StatusCode;
use actix_web::ResponseError;
use chrono::{DateTime, Utc};
use std::time::Instant;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("no user could be found with email {0:?}")]
    NoUserFound(String),
    #[error("The token expired at {0:?}")]
    TokenExpired(DateTime<Utc>),
    #[error("The token could not be parsed")]
    TokenParseError,
    #[error("The token could not be verified")]
    VerificationError,
    #[error(transparent)]
    JwtError(#[from] jwt::Error),
    #[error(transparent)]
    PasswordError(#[from] PasswordError),
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }
}
