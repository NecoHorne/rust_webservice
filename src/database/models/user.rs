use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    Unknown,
    User,
    Seller,
    Admin,
    Service,
    Cloud
}

#[derive(Debug,Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub uid: String,
    pub display_name: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub email_verified: bool,
    pub telephone: String,
    pub telephone_verified: bool,
    pub fcm: String,
    pub is_banned: bool,
    pub role: Role,
    pub password: String,
    pub meta_data: String,
    pub ip_address: String,

}

use super::schema::users;

#[derive( Debug, Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub uid: &'a str,
    pub display_name: &'a str,
    pub email: &'a str,
    pub role: i32,
    pub password: &'a str,
}