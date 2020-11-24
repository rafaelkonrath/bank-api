#[macro_use]
extern crate validator_derive;

mod config;
mod db;
mod errors;
mod handlers;
mod models;

use crate::config::Config;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use color_eyre::Result;
use handlers::app_config;
use tracing::{info, instrument};

use actix_web_prom::PrometheusMetrics;

#[actix_rt::main]
#[instrument]
async fn main() -> Result<()> {
    let config = Config::from_env().expect("Server configuration");

    let pool = config.db_pool().await.expect("Database configuration");

    let hashing = config.hashing();

    let params = config.params();

    info!("Starting server at http://{}:{}/", config.host, config.port);

    let prometheus = PrometheusMetrics::new("api", Some("/metrics"),None);

        HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .wrap(prometheus.clone())
                .data(pool.clone())
                .data(hashing.clone())
                .data(params.clone())
                .configure(app_config)
        })
        .bind(format!("{}:{}", config.host, config.port))?
        .run()
        .await?;

    Ok(())
}
