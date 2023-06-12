use std::error::Error;
use actix_web::{App, HttpServer, main};
use actix_web::body::BoxBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use actix_web::http::header::{CONTENT_TYPE, HeaderValue};
use actix_web::middleware::Logger;
use clap::Parser;
use tracing::log::LevelFilter;
use common::cli::CommonArgs;
use common::logging::init_logging;


mod home;


#[main]
async fn main() -> Result<(), Box<dyn Error>>{
    let cli = CommonArgs::parse();

    init_logging(&cli.logging);

    HttpServer::new(|| App::new()
        .wrap(Logger::default())
        .service(home::home)
    )
        .bind((cli.server.ip, cli.server.port))?
        .run()
        .await?;

    Ok(())
}
