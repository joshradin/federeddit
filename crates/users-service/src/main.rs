use actix_web::http::header::{Header, TryIntoHeaderValue};
use actix_web::middleware::Logger;
use actix_web::web::{Data, Query};
use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use argon2::password_hash::SaltString;
use chrono::Duration;
use clap::Parser;
use diesel::backend::Backend;
use diesel::r2d2::ConnectionManager;
use diesel::{Connection, MysqlConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use openssl::ssl::{SslAcceptor, SslMethod};
use r2d2::{ManageConnection, Pool};
use serde::Deserialize;
use std::env;
use std::error::Error;
use tracing::info;

use crate::actions::{create_user, login_user};
use crate::authenticator::{validate_token, Authenticator};
use crate::user::PublicUser;
use common::cli::{CommonArgs, SecurityArgs, SecurityBuilderError};
use common::logging::init_logging;
use users_api::auth::PasswordAuth;
use users_api::error::AuthError;
use users_api::header::Authorization;
use users_api::EmailAddress;

mod actions;
mod authenticator;
mod schema;
mod tokens;
mod user;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

type Database = Pool<ConnectionManager<MysqlConnection>>;

/// Launches the auth/user service
#[derive(Parser)]
struct Args {
    #[clap(flatten)]
    common: CommonArgs,
    #[clap(flatten)]
    security: SecurityArgs,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let path = dotenv().ok();

    let cli = Args::parse();

    init_logging(&cli.common.logging);
    info!("loaded {:?} into env", path);

    let secret = b"password";
    let authenticator = Data::new(Authenticator::<PublicUser>::new(secret));

    let passwords = PasswordAuth::new();

    let mut pool = establish_connection();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(authenticator.clone())
            .app_data(Data::new(passwords.clone()))
            .app_data(Data::new(pool.clone()))
            .service(validate_token)
            .service(create_user)
            .service(login_user)
    });
    let addr = (cli.common.server.ip, cli.common.server.port);
    let binded = match cli.security.ssl_acceptor() {
        Ok(security) => {
            info!("starting tls server at {:?}", addr);
            server.bind_openssl(addr, security)?
        }
        Err(SecurityBuilderError::NotPresent) => {
            info!("starting un-encrypted server at {:?}", addr);
            server.bind(addr)?
        }
        Err(e) => {
            return Err(Box::new(e) as Box<dyn Error>);
        }
    };

    binded.run().await?;

    Ok(())
}

fn establish_connection() -> Pool<ConnectionManager<MysqlConnection>> {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut manager = ConnectionManager::<MysqlConnection>::new(url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("should be a valid connection");

    {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        run_migrations(&mut conn).expect("could not run migrations");
    }

    pool
}

fn run_migrations<Db: Backend>(
    connection: &mut impl MigrationHarness<Db>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct GuestOptions {
    expires_after: Option<u64>,
}
