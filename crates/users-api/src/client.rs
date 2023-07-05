//! An auth service client

use crate::bearer::BearerToken;
use crate::header::Authorization;
use crate::user_service::{AuthenticatedUser, UserService};
use crate::User;
use actix_web::http::header::Header;
use async_trait::async_trait;
use common::utils::encode_base64;
use email_address::EmailAddress;
use reqwest::header::AUTHORIZATION;
use reqwest::Url;
use std::error::Error;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Client {
    host: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(host: String) -> Self {
        Self {
            host,
            client: reqwest::Client::default(),
        }
    }
}
#[derive(Debug, serde::Deserialize)]
struct UserInfo {
    username: String,
    email: EmailAddress,
}
#[async_trait]
impl UserService<RemoteUser> for Client {
    type Authenticated = AuthenticatedRemoteUser;
    type AuthError = reqwest::Error;

    async fn log_in(
        &self,
        user: &str,
        pass: &[u8],
    ) -> Result<Self::Authenticated, Self::AuthError> {
        let response = self
            .client
            .post(
                Url::from_str(&self.host)
                    .unwrap()
                    .join("/user/login")
                    .unwrap(),
            )
            .header(
                AUTHORIZATION,
                format!(
                    "Basic {}",
                    encode_base64(&format!("{user}:{}", std::str::from_utf8(pass).unwrap()))
                ),
            )
            .send()
            .await?;
        let auth = Authorization::from_str(&*String::from_utf8_lossy(
            response.headers().get(AUTHORIZATION).unwrap().as_bytes(),
        ))
        .unwrap();
        let info = response.json::<UserInfo>().await?;

        let remote_user = RemoteUser {
            client: self.clone(),
            username: info.username,
            email: info.email,
        };
        Ok(AuthenticatedRemoteUser {
            remote_user,
            bearer: auth.bearer().clone(),
        })
    }
}

#[derive(Debug)]
pub struct RemoteUser {
    client: Client,
    username: String,
    email: EmailAddress,
}

impl User for RemoteUser {
    fn username(&self) -> &str {
        &self.username
    }

    fn set_username(&mut self, name: &str) {
        todo!("can not change username")
    }

    fn email(&self) -> EmailAddress {
        self.email.clone()
    }
}

#[derive(Debug)]
pub struct AuthenticatedRemoteUser {
    remote_user: RemoteUser,
    bearer: BearerToken,
}

impl Deref for AuthenticatedRemoteUser {
    type Target = RemoteUser;

    fn deref(&self) -> &Self::Target {
        &self.remote_user
    }
}

impl DerefMut for AuthenticatedRemoteUser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.remote_user
    }
}
impl AuthenticatedUser<RemoteUser> for AuthenticatedRemoteUser {
    fn bearer(&self) -> &BearerToken {
        &self.bearer
    }
}
