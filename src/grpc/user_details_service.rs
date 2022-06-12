use tonic::{Request, Response, Status};
use auth::user_details_service_server::{UserDetailsService as UserDetails};
use crate::grpc::auth;

#[derive(Debug, Default)]
pub struct UserDetailsService {}

#[tonic::async_trait]
impl UserDetails for UserDetailsService {

    ///
    /// Get public user details
    ///
    async fn get_user_details(&self, request: Request<auth::GetUserDetailsRequest>) -> Result<Response<auth::GetUserDetailsResponse>, Status>
    {
        println!("Request: {:?}", request);
        let req = request.into_inner();
        //TODO Process actual request data

        let user_details = Some(auth::UserDetails{
            uid: "123".to_owned(),
            display_name: "test".to_owned(),
            user_rating: 0.0
        });

        let reply = auth::GetUserDetailsResponse{
            success: true,
            message: "test".to_owned(),
            user_details
        };
        Ok(Response::new(reply))
    }

    ///
    /// Get public seller details
    ///
    async fn get_seller_details(&self, request: Request<auth::GetSellerDetailsRequest>) -> Result<Response<auth::GetSellerDetailsResponse>, Status>
    {
        println!("Request: {:?}", request);
        let req = request.into_inner();
        //TODO Process actual request data
        let seller_details = Some(
            auth::SellerDetails{
                uid: "".to_owned(),
                display_name: "".to_owned(),
                user_rating: 0.0,
                company_name: "".to_owned(),
                description: "".to_owned(),
                profile_picture: "".to_owned(),
                shop_url: "".to_owned(),
            }
        );


        let reply = auth::GetSellerDetailsResponse{
            success: true,
            message: "test message".to_owned(),
            seller_details
        };
        Ok(Response::new(reply))
    }

    ///
    /// Register a New user
    ///
    async fn register_new_user(&self, request: Request<auth::RegisterNewUserRequest>) -> Result<Response<auth::RegisterNewUserResponse>, Status>
    {
        println!("Request: {:?}", request);
        let req = request.into_inner();
        //TODO Process actual request data

        let reply = auth::RegisterNewUserResponse{
            success: true,
            response_message: "".to_owned()
        };
        Ok(Response::new(reply))
    }

    ///
    /// Update the user Firebase Cloud Messaging token (FCM)
    ///
    async fn update_user_fcm(&self, request: Request<auth::UpdateUserFcmRequest>) -> Result<Response<auth::UpdateUserFcmResponse>, Status>
    {
        println!("Request: {:?}", request);
        let req = request.into_inner();
        //TODO Process actual request data

        let reply = auth::UpdateUserFcmResponse{
            success: true,
            response_message: "".to_owned()
        };
        Ok(Response::new(reply))
    }

    ///
    /// Update the user data (should only be possible by admin or the specific user who's details are being updated)
    ///
    async fn update_user_details(&self, request: Request<auth::UpdateUserDetailsRequest>) -> Result<Response<auth::UpdateUserDetailsResponse>, Status>
    {
        println!("Request: {:?}", request);
        let req = request.into_inner();
        //TODO Process actual request data

        let reply = auth::UpdateUserDetailsResponse{
            success: true,
            response_message: "".to_owned()
        };
        Ok(Response::new(reply))
    }

}