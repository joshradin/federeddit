use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::time::{Instant, SystemTime};
use reqwest::Url;
use actix_web::guard::{Guard, GuardContext};
use log::error;
use crate::auth::AuthService;
use crate::bearer::BearerToken;
use crate::error::AuthError;
use crate::ExpirationTime;
use crate::header::Authorization;

/// Middle ware checker
#[derive(Debug)]
pub struct AuthorizationGuard<A : AuthService> {
    validated_tokens: Arc<RwLock<HashMap<BearerToken, ExpirationTime>>>,
    auth_endpoint: A
}

impl<A: AuthService> AuthorizationGuard<A> {
    pub fn new(validated_tokens: Arc<RwLock<HashMap<BearerToken, ExpirationTime>>>, auth_endpoint: A) -> Self {
        Self { validated_tokens, auth_endpoint }
    }
}

impl<A : AuthService> Guard for AuthorizationGuard<A> {
    fn check(&self, ctx: &GuardContext<'_>) -> bool {
        match ctx.header::<Authorization>() {
            None => { false }
            Some(auth) => {
                let bearer = auth.bearer();

                let mut remove = false;
                if let Some(expires) = self.validated_tokens.read().get(bearer) {
                    if expires < &ExpirationTime::from(SystemTime::now()) {
                        return true
                    } else {
                        remove = true;
                    }
                }
                if remove {
                    self.validated_tokens.write().remove(bearer);
                }

                match self.auth_endpoint.validate_token(bearer) {
                    Ok(expires) => {
                        self.validated_tokens.write().insert(bearer.clone(), expires);
                        true
                    }
                    Err(error) => {
                        error!("auth error: {}", error);
                        false
                    }
                }
            }
        }
    }
}
