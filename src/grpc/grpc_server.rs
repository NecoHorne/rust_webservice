use tonic::transport::Server;
use services::auth_service::*;
use services::auth_service::auth::authentication_service_server::{AuthenticationServiceServer as AuthenticationServer};

mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let auth_service = AuthenticationService::default();

    Server::builder()
        .add_service(AuthenticationServer::new(auth_service))
        .serve(addr)
        .await?;

    Ok(())
}