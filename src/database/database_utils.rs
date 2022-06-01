use dotenv::dotenv;
use std::env;
use diesel::prelude::*;

use crate::database::models::{NewPost, Post};
use crate::database::schema::posts;

pub fn establish_connection() -> MysqlConnection {

    dotenv().ok(); //Load the env file

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &MysqlConnection, title: &str, body: &str) -> Post {

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");

    posts::table.order(posts::id.desc()).first(conn).unwrap()
}