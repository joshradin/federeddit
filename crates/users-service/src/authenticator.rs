//! Authenticates!

use crate::tokens::AuthenticatedUserToken;
use crate::user::PublicUser;
use actix_web::http::header::Header;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json};
use actix_web::{get, HttpRequest, Responder};
use chrono::{DateTime, Duration, Utc};
use hmac::digest::KeyInit;
use hmac::Hmac;
use jwt::{Error, SignWithKey, VerifyWithKey, VerifyingAlgorithm};
use sha2::Sha384;
use std::marker::PhantomData;
use std::time::SystemTime;
use tracing::{info, instrument};
use users_api::bearer::BearerToken;
use users_api::error::AuthError;
use users_api::header::Authorization;
use users_api::{ExpirationTime, User};

/// Used for authenticating
#[derive(Debug, Clone)]
pub struct Authenticator<U: User> {
    hmac: Hmac<Sha384>,
    _user: PhantomData<U>,
}

impl<U: User> Authenticator<U> {
    /// Created a token factory
    pub fn new(secret: &[u8]) -> Self {
        let hmac = Hmac::new_from_slice(secret).unwrap();
        Self {
            hmac,
            _user: PhantomData,
        }
    }

    /// Creates a token for a user
    pub fn create_token(
        &self,
        user: &PublicUser,
        expires_in: Duration,
    ) -> Result<BearerToken, AuthError> {
        let token = AuthenticatedUserToken::with_valid_duration(user, expires_in);
        token
            .sign_with_key(&self.hmac)
            .map(|s| BearerToken::from(s))
            .map_err(|e| e.into())
    }

    /// Validates the token

    pub fn validate_token(&self, bearer: &BearerToken) -> Result<ExpirationTime, AuthError> {
        let tok: AuthenticatedUserToken =
            String::from_utf8_lossy(bearer.as_ref()).verify_with_key(&self.hmac)?;

        let now = DateTime::<Utc>::from(SystemTime::now());
        info!(
            "checking if bearer has expired... (expires: {}, now: {})",
            tok.expiration_time(),
            now
        );

        if tok.expiration_time() < now {
            return Err(AuthError::TokenExpired(tok.expiration_time()));
        }

        Ok(tok.expiration_time())
    }
}

#[get("/")]
#[instrument]
pub async fn validate_token(
    auth: Data<Authenticator<PublicUser>>,
    req: HttpRequest,
) -> Result<Json<ExpirationTime>, AuthError> {
    let Ok(auth_header) = Authorization::parse(&req) else {
        return Err(AuthError::TokenParseError)
    };

    let bearer = auth_header.bearer();
    auth.validate_token(bearer).map(Json)
}

#[cfg(test)]
mod tests {
    use crate::authenticator::Authenticator;
    use chrono::Duration;
    use users_api::{EmailAddress, User};
    use crate::user::PublicUser;

    #[test]
    fn validate_a_token() {


        let auth = Authenticator::<PublicUser>::new(b"password");
        let bearer = auth.create_token(&PublicUser::new(EmailAddress::new_unchecked("test"), "test".to_string()), Duration::days(30)).unwrap();
        auth.validate_token(&bearer).expect("couldn't verify");
    }
}
