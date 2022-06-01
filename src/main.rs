#[macro_use] extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod database;
mod controllers;
use controllers::controller_one::*;

#[rocket::main]
async fn main() {
    let _ = rocket::build().mount("/", routes![index, test, hello]).launch().await.expect("TODO: panic message");
}