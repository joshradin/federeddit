//! Used for common cli components

use clap::{ArgAction, Args, Parser};
use openssl::error::ErrorStack;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslContext, SslFiletype, SslMethod};
use openssl::x509::X509;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct LoggingArgs {
    /// How verbose logging should be.
    #[clap(short = 'v')]
    #[clap(action = ArgAction::Count)]
    #[clap(value_parser = clap::value_parser!(u8).range(0..3))]
    pub verbosity: u8,
}

#[derive(Debug, Parser)]
pub struct WebServerArgs {
    #[clap(long, default_value = "localhost")]
    pub ip: String,
    #[clap(long, default_value_t = 8080)]
    pub port: u16,
}

#[derive(Debug, Parser)]
pub struct CommonArgs {
    /// Contains the logging args
    #[clap(flatten)]
    pub logging: LoggingArgs,
    /// Contains web server args.
    #[clap(flatten)]
    pub server: WebServerArgs,
}

/// Used for security purposes, usually for TLS.
#[derive(Debug, Parser)]
pub struct SecurityArgs {
    /// Path to a certificate (public key)
    #[clap(long)]
    #[clap(requires("cert_key"))]
    cert: Option<PathBuf>,
    /// Path to a private key.
    ///
    /// Required if --cert is specified.
    #[clap(long)]
    #[clap(alias("key"))]
    cert_key: Option<PathBuf>,
    /// Path to a root authority
    #[clap(long)]
    capath: Option<PathBuf>,
}

impl SecurityArgs {
    /// Creates an ssl context from the security args
    pub fn ssl_acceptor(&self) -> Result<SslAcceptorBuilder, SecurityBuilderError> {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
        if let Some(cert) = &self.cert {
            let cery_key = self
                .cert_key
                .as_ref()
                .expect("will always be present if cert is present");

            builder.set_certificate_file(cert, SslFiletype::PEM)?;
            builder.set_private_key_file(cery_key, SslFiletype::PEM)?;
        } else {
            return Err(SecurityBuilderError::NotPresent);
        }

        if let Some(ca) = &self.capath {
            builder.set_ca_file(ca)?;
        }

        Ok(builder)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SecurityBuilderError {
    #[error("No ssl context was built")]
    NotPresent,
    #[error(transparent)]
    OpensslError(#[from] ErrorStack),
}
