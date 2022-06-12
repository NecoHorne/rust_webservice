use std::env;
use tonic::transport::Server;
use crate::grpc::auth_service::*;
use crate::grpc::user_details_service::*;
use crate::grpc::auth;
use auth::authentication_service_server::{AuthenticationServiceServer as AuthenticationServer};
use auth::user_details_service_server::{UserDetailsServiceServer as UserDetailsServer};

#[tokio::main]
pub async fn start_grpc_server() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("GRPC_PORT").unwrap_or("50051".to_string());
    let addr = format!("[::1]:{}", port).parse()?;
    let auth_service = AuthenticationService::default();
    let user_details_service = UserDetailsService::default();

    println!("grpc server starting on port {}", port);
    Server::builder()
        .add_service(AuthenticationServer::new(auth_service))
        .add_service(UserDetailsServer::new(user_details_service))
        .serve(addr)
        .await?;
    Ok(())
}