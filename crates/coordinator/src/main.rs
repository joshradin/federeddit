use actix_web::body::BoxBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use actix_web::http::header::{HeaderValue, CONTENT_TYPE};
use actix_web::middleware::Logger;
use actix_web::{main, App, HttpServer};
use clap::Parser;
use common::cli::CommonArgs;
use common::logging::init_logging;
use std::error::Error;
use tracing::log::LevelFilter;

mod home;

#[main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = CommonArgs::parse();

    init_logging(&cli.logging);

    HttpServer::new(|| App::new().wrap(Logger::default()).service(home::home))
        .bind((cli.server.ip, cli.server.port))?
        .run()
        .await?;

    Ok(())
}
