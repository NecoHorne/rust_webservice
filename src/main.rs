#[macro_use] extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;

mod database;
mod grpc;

mod webserver;
use webserver::server::rocket_main;

use std::thread;
use grpc::grpc_server::grpc_server;

fn main(){

    thread::spawn(|| {
        grpc_server().expect("gRPC server panic");
    });

    rocket_main().expect("Webserver panic");

}