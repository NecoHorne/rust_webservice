use rocket::fs::FileServer;
use rocket::Request;
use rocket_dyn_templates::{context, Template};
use crate::webserver::controllers::controller_one::*;
use crate::webserver::controllers::controller_two::*;


#[catch(500)]
fn internal_error() -> Template {
    Template::render("500", context! {
        page_title: "500 Error",
        title: "Whoops! Looks like we messed up."
    })
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    Template::render("404", context! {
        page_title: "404 not Found",
        title: format!("I couldn't find '{}'. Try something else?", req.uri())
    })
}

#[rocket::main]
pub async fn start_web_server() -> Result<(), Box<dyn std::error::Error>>{
    let _ = rocket::build()
        .mount("/", routes![index, test, hello, index_two, test_two, hello_two])
        .mount("/static", FileServer::from("static"))
        .register("/", catchers![internal_error, not_found])
        .attach(Template::fairing())
        .launch().await?;
    Ok(())
}