mod config;
mod db;
mod handlers;
mod models;
mod errors;

use crate::handlers::*;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::io;
use tokio_postgres::NoTls;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    let pool = config.db.create_pool(NoTls).unwrap();

    println!(
        "Starting server at http://{}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(status)
            .service(get_posts)
            .service(create_post)
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
