use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use crate::webserver::controller_one::*;
use crate::webserver::controller_two::*;

#[rocket::main]
pub async fn start_web_server() -> Result<(), Box<dyn std::error::Error>>{

    let _ = rocket::build()
        .mount("/", routes![index, test, hello, index_two, test_two, hello_two])
        .mount("/static", FileServer::from("static"))
        .attach(Template::fairing())
        .launch().await?;

    Ok(())
}