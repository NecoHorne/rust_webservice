pub mod server;
mod auth_service;
mod user_details_service;
mod auth {
    tonic::include_proto!("auth");
}