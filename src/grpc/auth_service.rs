use tonic::{Request, Response, Status};
pub mod auth {
    tonic::include_proto!("auth");
}
use auth::authentication_service_server::{AuthenticationService as Authentication};

#[derive(Debug, Default)]
pub struct AuthenticationService {}

#[tonic::async_trait]
impl Authentication for AuthenticationService {

    async fn authenticate_user(&self, request: Request<auth::AuthenticateUserRequest>) -> Result<Response<auth::AuthenticateUserResponse>, Status>
    {
        println!("Request: {:?}", request);

        let req = request.into_inner();
        //todo process actual request data

        let reply = auth::AuthenticateUserResponse {
            success: true,
            token: "1234".to_owned(),
            refresh_token: "1234".to_owned(),
            response_message: "This is a response message".to_owned(),
            email: "test@test.com".to_owned(),
            email_verified: true,
            uid: "1234".to_owned(),
            photo_url: "".to_owned(),
            meta_data: "".to_owned(),
            display_name: "".to_owned()
        };

        Ok(Response::new(reply))
    }

    async fn authenticate_service(&self , request: Request<auth::AuthenticateServiceRequest>) -> Result<Response<auth::AuthenticateServiceResponse>, Status>
    {
        println!("Request: {:?}", request);
        let req = request.into_inner();
        //todo process actual request data

        let reply = auth::AuthenticateServiceResponse{
            success: true,
            token: "1234".to_owned(),
            response_message: "This is a response message".to_owned()
        };
        Ok(Response::new(reply))
    }

    async fn check_token(&self , request: Request<auth::CheckTokenRequest>) -> Result<Response<auth::CheckTokenResponse>, Status>
    {
        println!("Request: {:?}", request);
        let req = request.into_inner();
        //todo process actual request data

        let reply = auth::CheckTokenResponse{
            authenticated: true,
            response_message: "This is a response message".to_owned()
        };
        Ok(Response::new(reply))
    }

    async fn token_login(&self , request: Request<auth::TokenLoginRequest>) -> Result<Response<auth::CheckTokenResponse>, Status>
    {
        println!("Request: {:?}", request);
        let req = request.into_inner();
        //todo process actual request data

        let reply = auth::CheckTokenResponse{
            authenticated: true,
            response_message: "This is a response message".to_owned()
        };
        Ok(Response::new(reply))
    }

    async fn refresh_token(&self , request: Request<auth::RefreshTokenRequest>) -> Result<Response<auth::RefreshTokenResponse>, Status>
    {
        println!("Request: {:?}", request);
        let req = request.into_inner();
        //todo process actual request data

        let reply = auth::RefreshTokenResponse{
            success: true,
            token: "1234".to_owned(),
            response_message: "This is a response message".to_owned()
        };
        Ok(Response::new(reply))
    }

    async fn get_user_by_token(&self , request: Request<auth::GetUserByTokenRequest>) -> Result<Response<auth::GetsUserByTokenResponse>, Status>
    {
        println!("Request: {:?}", request);
        let req = request.into_inner();
        //todo process actual request data

        let reply = auth::GetsUserByTokenResponse{
            success: true,
            token: "1234".to_owned(),
            refresh_token: "1234".to_owned(),
            response_message: "Response message".to_owned(),
            email: "test@test.com".to_owned(),
            email_verified: true,
            uid: "1234".to_owned(),
            photo_url: "".to_owned(),
            meta_data: "".to_owned(),
            display_name: "".to_owned(),
            fcm: "".to_owned()
        };
        Ok(Response::new(reply))
    }

    async fn reset_password(&self , request: Request<auth::ResetPasswordRequest>) -> Result<Response<auth::ResetPasswordResponse>, Status>
    {
        println!("Request: {:?}", request);
        let req = request.into_inner();
        //todo process actual request data

        let reply = auth::ResetPasswordResponse{
            response_message: "Resposne message".to_owned()
        };
        Ok(Response::new(reply))
    }

    async fn invalidate_refresh_token(&self , request: Request<auth::InvalidateRefreshTokenRequest>) -> Result<Response<auth::InvalidateRefreshTokenResponse>, Status>
    {
        println!("Request: {:?}", request);
        let req = request.into_inner();
        //todo process actual request data

        let reply = auth::InvalidateRefreshTokenResponse{
            success: true
        };
        Ok(Response::new(reply))
    }

    async fn verify_user_email(&self , request: Request<auth::VerifyUserEmailRequest>) -> Result<Response<auth::VerifyUserEmailResponse>, Status>
    {
        println!("Request: {:?}", request);
        let req = request.into_inner();
        //todo process actual request data

        let reply = auth::VerifyUserEmailResponse{
            success: true,
            message: "response message".to_owned()
        };
        Ok(Response::new(reply))
    }

    async fn resend_verify_email(&self , request: Request<auth::ResendVerifyEmailRequest>) -> Result<Response<auth::ResendVerifyEmailResponse>, Status>
    {
        println!("Request: {:?}", request);
        let req = request.into_inner();
        //todo process actual request data

        let reply = auth::ResendVerifyEmailResponse{
            success: true,
            message: "this is a response message".to_owned()
        };
        Ok(Response::new(reply))
    }

    async fn hash_password(&self , request: Request<auth::HashPasswordRequest>) -> Result<Response<auth::HashPasswordResponse>, Status>
    {
        println!("Request: {:?}", request);
        let req = request.into_inner();
        //todo process actual request data

        let reply = auth::HashPasswordResponse{
            hash: "hashedpassword".to_owned(),
            message: "message".to_owned(),
            success: true
        };
        Ok(Response::new(reply))
    }

}



