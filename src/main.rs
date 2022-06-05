#[macro_use] extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;

mod database;
mod controllers;

use controllers::controller_one::*;
use controllers::controller_two::*;

#[rocket::main]
async fn main(){
    let _ = rocket::build().mount("/",
                                  routes![index, test, hello, index_two, test_two, hello_two]
    ).launch().await.expect("TODO: panic message");
}