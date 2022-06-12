use crate::diesel::prelude::*;
use crate::database::models::schema::posts::dsl::*;
use crate::database::database_utils::establish_connection;
use crate::database::models::posts::Post;

#[get("/")]
pub fn index_two() -> &'static str {
    "Hello API"
}

#[get("/hello/<name>")]
pub fn hello_two(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/test")]
pub fn test_two() -> String {

    let connection = establish_connection();

    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());

    let mut response_string: String =  String::from("");

    for post in results {
        response_string.push_str("----------\n");
        response_string.push_str(&post.title);
        response_string.push_str("\n");
        response_string.push_str(&post.body);
        response_string.push_str("\n");
    }

    return response_string;
}