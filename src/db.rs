use crate::models::{Post};
use crate::errors::{AppError, AppErrorType};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;


pub async fn get_posts(client: &Client) -> Result<Vec<Post>, AppError> {

    let statement = client
        .prepare("select * from post order by id desc limit 10")
        .await
        .map_err(|err| AppError{ message: None, cause: Some(err.to_string()), error_type: AppErrorType::DbError })?;

    let todos = client.query(&statement, &[])
        .await
        .expect("Error getting posts")
        .iter()
        .map(|row| Post::from_row_ref(row).unwrap())
        .collect::<Vec<Post>>();

    Ok(todos)
}

pub async fn create_post(client: &Client, title: String, subtitle: String, image_url: String) -> Result<Post, AppError> {
    let statement = client.prepare("insert into post (title, subtitle, image_url) values ($1, $2, $3) returning id, title, subtitle, image_url").await.unwrap();

    client.query(&statement, &[&title, &subtitle, &image_url])
        .await
        .expect("Error creating post")
        .iter()
        .map(|row| Post::from_row_ref(row).unwrap())
        .collect::<Vec<Post>>()
        .pop()
        .ok_or(AppError {
            message: Some("Error creating post".to_string()),
            cause: Some("Unknown error".to_string()),
            error_type: AppErrorType::DbError
        })
}

