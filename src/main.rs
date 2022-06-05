#[macro_use] extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate rocket_dyn_templates;
extern crate tera;

mod database;
mod webserver;
mod grpc;

use grpc::server::start_grpc_server;
use webserver::server::start_web_server;
use std::thread;

fn main(){

    /*
     * Not sure if this would be the correct way to do this, I need to do some more reading.
     * Currently to get the gRPC server running along with the rocket webserver I have to spawn one
     * in a different thread.
     */
    thread::spawn(|| {
        start_grpc_server().expect("gRPC server panic");
    });

    start_web_server().expect("Webserver panic");

}