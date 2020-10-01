use deadpool_postgres::Pool;
use slog::Logger;
use crate::db;
use crate::errors::{AppError};
use crate::models::*;
use actix_web::{get, post, web, HttpResponse, Responder};
use deadpool_postgres::{Client};
use slog::{o, crit};

pub async fn get_client(pool: Pool, log: Logger) -> Result<Client, AppError> {
    pool.get().await.map_err(|err| {
        let sublog = log.new(o!("cause" => err.to_string()));

        crit!(sublog, "Error creating client.");

        AppError::db_error(err)
    })
}

#[get("/")]
pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "UP".to_string(),
    })
}

#[get("/posts{_:/?}")]
pub async fn get_posts(state: web::Data<AppState>) -> Result<impl Responder, AppError> {

    let log = state.log.new(o!("handler" => "get_posts"));

    let client: Client = get_client(state.pool.clone(), log).await?;

    let result = db::get_posts(&client).await;

    result.map(|todos| HttpResponse::Ok().json(todos))
}

#[post("/posts{_:/?}")]
pub async fn create_post(state: web::Data<AppState>, json: web::Json<CreatePost>) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "create_posts"));

    let client: Client = get_client(state.pool.clone(), log).await?;

    let result = db::create_post(
        &client,
        json.title.clone(),
        json.subtitle.clone(),
        json.image_url.clone(),
    )
    .await;

    result.map(|post| HttpResponse::Ok().json(post))
}
