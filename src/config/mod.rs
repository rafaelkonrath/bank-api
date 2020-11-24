pub mod crypto;
pub mod params;

use color_eyre::Result;
use dotenv::dotenv;
use eyre::WrapErr;

use serde::Deserialize;
use sqlx::postgres::PgPool;

use crypto::CryptoService;
use params::Params;
use std::sync::Arc;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub database_url: String,
    pub secret_key: String,
    pub jwt_secret: String,
    pub client_id: String,
    pub client_secret: String,
    pub token_uri: String,
    pub redirect_uri: String,
    pub auth_uri: String,
    pub api_uri: String,
}


impl Config {
    #[instrument]
    pub fn from_env() -> Result<Config> {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        info!("Loading configuration");

        let mut c = config::Config::new();

        c.merge(config::Environment::default())?;

        c.try_into()
            .context("loading configuration from environment")
    }

    #[instrument(skip(self))]
    pub async fn db_pool(&self) -> Result<PgPool> {
        info!("Creating database connection pool.");
        PgPool::builder()
            .connect_timeout(std::time::Duration::from_secs(30))
            .build(&self.database_url)
            .await
            .context("creating database connection pool")
    }

    #[instrument(skip(self))]
    pub fn hashing(&self) -> CryptoService {
        CryptoService {
            key: Arc::new(self.secret_key.clone()),
            jwt_secret: Arc::new(self.jwt_secret.clone()),
        }
    }

    #[instrument(skip(self))]
    pub fn params(&self) -> Params {
        Params {
            client_id: Arc::new(self.client_id.clone()),
            client_secret: Arc::new(self.client_secret.clone()),
            token_uri: Arc::new(self.token_uri.clone()),
            redirect_uri: Arc::new(self.redirect_uri.clone()),
            auth_uri: Arc::new(self.auth_uri.clone()),
            api_uri: Arc::new(self.auth_uri.clone()),
        }
    }
}
