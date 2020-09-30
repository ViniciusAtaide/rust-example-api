use crate::db;
use crate::errors::{AppError, AppErrorType};
use crate::models::*;
use actix_web::{get, post, web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};

#[get("/")]
pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "UP".to_string(),
    })
}

#[get("/posts{_:/?}")]
pub async fn get_posts(db_pool: web::Data<Pool>) -> Result<impl Responder, AppError> {
    let client: Client = db_pool.get().await.map_err(AppError::db_error)?;

    let result = db::get_posts(&client).await;

    result.map(|todos| HttpResponse::Ok().json(todos))
}

#[post("/posts{_:/?}")]
pub async fn create_post(db_pool: web::Data<Pool>, json: web::Json<CreatePost>) -> Result<impl Responder, AppError> {
    let client: Client = db_pool
        .get()
        .await
        .map_err(AppError::db_error)?;

    let result = db::create_post(
        &client,
        json.title.clone(),
        json.subtitle.clone(),
        json.image_url.clone(),
    )
    .await;

    result.map(|post| HttpResponse::Ok().json(post))
}
