use crate::webserver::controller_one::*;
use crate::webserver::controller_two::*;

#[rocket::main]
pub async fn rocket_main()-> Result<(), Box<dyn std::error::Error>>{

    let _ = rocket::build().mount("/", routes![index, test, hello, index_two, test_two, hello_two]
    ).launch().await?;

    Ok(())
}