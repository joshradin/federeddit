//! Common actions

use crate::authenticator::Authenticator;
use crate::user::PublicUser;
use crate::Database;
use actix_web::http::header::AUTHORIZATION;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json};
use actix_web::{error, post, web, HttpRequest, HttpResponse, Responder};
use base64::alphabet::URL_SAFE;
use base64::engine::general_purpose::PAD;
use base64::engine::GeneralPurpose;
use base64::Engine;
use diesel::QueryDsl;
use hmac::digest::typenum::op;
use serde::Deserialize;
use std::error::Error;
use chrono::Duration;
use tracing::instrument;
use users_api::auth::PasswordAuth;
use users_api::error::AuthError;

#[derive(Debug, Deserialize)]
struct CreateUserBody {
    email: String,
    username: String,
    password: String,
}

#[post("user/create")]
pub async fn create_user(
    create_user: Json<CreateUserBody>,
    password_hasher: Data<PasswordAuth>,
    cnxn: Data<Database>,
) -> actix_web::Result<impl Responder> {
    web::block(move || {
        let mut conn = cnxn.get().expect("couldn't get db connection from pool");

        let hashed = password_hasher
            .hash_password(&create_user.password.as_bytes())
            .map_err(|e| Box::new(e) as Box<dyn Error + Send>)?;
        PublicUser::create_new_user(
            &mut conn,
            &create_user.email,
            &create_user.username,
            &hashed,
        )
        .map_err(|e| Box::new(e) as Box<dyn Error + Send>)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok())
}

#[post("user/login")]
#[instrument]
pub async fn login_user(
    req: HttpRequest,
    password_hasher: Data<PasswordAuth>,
    auth: Data<Authenticator<PublicUser>>,
    cnxn: Data<Database>,
) -> actix_web::Result<impl Responder> {
    let (email, password) = if let Some(header_value) = req.headers().get(AUTHORIZATION) {
        if header_value.as_bytes().starts_with(b"Bearer ") {
            todo!("bearer re-auth")
        } else if header_value.as_bytes().starts_with(b"Basic ") {
            let basic_auth = header_value
                .as_bytes()
                .strip_prefix(b"Basic ")
                .ok_or(error::ErrorNotAcceptable("bad auth"))?;

            let decoder = GeneralPurpose::new(&URL_SAFE, PAD);
            let result = decoder
                .decode(basic_auth)
                .map_err(error::ErrorNotAcceptable)
                .and_then(|vec| String::from_utf8(vec).map_err(error::ErrorNotAcceptable))?;

            let (email, password) = result
                .split_once(":")
                .ok_or(error::ErrorNotAcceptable("bad auth"))?;
            (email.to_string(), password.to_string())
        } else {
            return Err(error::ErrorNotAcceptable("invalid authorization scheme"));
        }
    } else {
        return Err(error::ErrorNotAcceptable("no AUTHORIZATION header"));
    };

    let user = web::block(move || -> Result<PublicUser, AuthError> {
        let mut conn = cnxn.get().expect("could not get db connection");
        let user =
            PublicUser::get_user(&mut conn, &email).ok_or(AuthError::NoUserFound(email.clone()))?;

        user.verify_password(&mut conn, &password_hasher, &password)?;

        Ok(user)
    })
    .await??;

    let token  = auth.create_token(&user, Duration::days(30))?;

    Ok(HttpResponse::Ok().insert_header((AUTHORIZATION, token.to_string())).finish())
}
