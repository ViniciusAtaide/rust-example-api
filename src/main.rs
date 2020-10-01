mod config;
mod db;
mod errors;
mod handlers;
mod models;

use crate::handlers::*;
use crate::models::AppState;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use slog::{info, o, Drain, Logger};
use slog_term;
use std::io;
use tokio_postgres::NoTls;

fn configure_log() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
    let console_drain = slog_async::Async::new(console_drain).build().fuse();
    slog::Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();
    let pool = config.db.create_pool(NoTls).unwrap();

    let log = configure_log();

    info!(
        log,
        "Starting server at http://{}:{}", config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            .data(AppState { 
                pool: pool.clone(),
                log: log.clone()
            })
            .service(status)
            .service(get_posts)
            .service(create_post)
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
