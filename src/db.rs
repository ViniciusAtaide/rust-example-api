use crate::models::{Post};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io;


pub async fn get_posts(client: &Client) -> Result<Vec<Post>, io::Error> {

    let statement = client.prepare("select * from post order by id desc limit 10").await.unwrap();

    let todos = client.query(&statement, &[])
        .await
        .expect("Error getting posts")
        .iter()
        .map(|row| Post::from_row_ref(row).unwrap())
        .collect::<Vec<Post>>();

    Ok(todos)
}

pub async fn create_post(client: &Client, title: String, subtitle: String, image_url: String) -> Result<Post, io::Error> {
    let statement = client.prepare("insert into post (title, subtitle, image_url) values ($1, $2, $3) returning id, title, subtitle, image_url").await.unwrap();

    client.query(&statement, &[&title, &subtitle, &image_url])
        .await
        .expect("Error creating post")
        .iter()
        .map(|row| Post::from_row_ref(row).unwrap())
        .collect::<Vec<Post>>()
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Error creating post"))
}

