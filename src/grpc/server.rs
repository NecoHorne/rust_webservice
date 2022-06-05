use tonic::transport::Server;
use crate::grpc::auth_service::*;
use auth::authentication_service_server::{AuthenticationServiceServer as AuthenticationServer};

#[tokio::main]
pub async fn start_grpc_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let auth_service = AuthenticationService::default();

    println!("grpc server starting on port 50051");
    Server::builder()
        .add_service(AuthenticationServer::new(auth_service))
        .serve(addr)
        .await?;

    Ok(())
}