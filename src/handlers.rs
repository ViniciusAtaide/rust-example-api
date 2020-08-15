use crate::models::*;
use crate::db;
use actix_web::{web, Responder, HttpResponse};
use deadpool_postgres::{Pool, Client};

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status { status: "UP".to_string() })
}

pub async fn get_posts(db_pool: web::Data<Pool>) -> impl Responder {

    let client: Client = db_pool.get().await.expect("Error connecting to the database");

    let result = db::get_posts(&client).await;

    match result {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn create_post(db_pool: web::Data<Pool>, json: web::Json<CreatePost>) -> impl Responder {

    let client: Client =
        db_pool.get().await.expect("Error connecting to the database");

    let result = db::create_post(&client, json.title.clone(), json.subtitle.clone(), json.image_url.clone()).await;

    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into()
    }

}