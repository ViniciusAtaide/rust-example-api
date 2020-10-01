use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use slog::Logger;
use deadpool_postgres::Pool;

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

pub struct AppState {
    pub pool: Pool,
    pub log: Logger
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "post")]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub subtitle: String,
    pub image_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub subtitle: String,
    pub image_url: String, 
}
#[derive(Serialize)]
pub struct ResultCheck {
    pub success: bool
}