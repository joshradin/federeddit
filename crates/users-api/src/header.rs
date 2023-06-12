//! Used for the auth header

use crate::bearer::BearerToken;
use crate::error::AuthError;
use actix_web::error::ParseError;
use actix_web::http::header::{
    Header, HeaderName, HeaderValue, InvalidHeaderValue, TryIntoHeaderValue,
};
use actix_web::HttpMessage;
use log::{error, info};
use nom::branch::alt;
use nom::bytes;
use nom::bytes::complete::{tag, take_till, take_until};
use nom::combinator::{complete, eof};
use nom::error::ErrorKind;
use nom::multi::many0;
use nom::sequence::{preceded, terminated};

/// Authorization header
#[derive(Debug)]
pub struct Authorization {
    bearer: BearerToken,
}

impl Authorization {
    /// Create a new authorization header from a bearer token
    pub fn new(bearer: BearerToken) -> Self {
        Self { bearer }
    }

    /// Gets the bearer token
    pub fn bearer(&self) -> &BearerToken {
        &self.bearer
    }
}

impl TryIntoHeaderValue for Authorization {
    type Error = InvalidHeaderValue;

    fn try_into_value(self) -> Result<HeaderValue, Self::Error> {
        HeaderValue::from_str(&self.bearer.to_string())
    }
}

impl Header for Authorization {
    fn name() -> HeaderName {
        HeaderName::from_static("authorization")
    }

    fn parse<M: HttpMessage>(msg: &M) -> Result<Self, ParseError> {
        match msg.headers().get(Authorization::name()) {
            None => Err(ParseError::Incomplete),
            Some(value) => {
                let as_string = String::from_utf8_lossy(value.as_bytes());
                info!("auth header value: {}", as_string);

                let bearer = as_string.replace("Bearer ", "");

                let bearer = BearerToken::from(bearer);
                Ok(Authorization { bearer })
            }
        }
    }
}
