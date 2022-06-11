use std::collections::HashMap;
use redis::Commands;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InvalidatedToken{
    token: String,
    user_id: String,
    expiry_date: String
}

impl InvalidatedToken{

    ///
    /// Constructor for invalidated token
    ///
    pub fn new(token: String, user_id: String, expiry_date: String) -> Self{
        InvalidatedToken{
            token,
            user_id,
            expiry_date
        }
    }

    pub fn get_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /*
     * Redis Methods
     */

    pub fn redis_key(&self) -> String {
        format!("invalidated_token:{}", &self.token)
    }

    pub fn save_token(&self, con: &mut redis::Connection) {
        redis::cmd("HMSET")
            .arg(&self.redis_key())
            .arg(&[("token", &self.token), ("user_id", &self.user_id), ("expiry_date", &self.expiry_date)])
            .execute(con)
    }

    pub fn delete_token(&self, con: &mut redis::Connection) {
        if InvalidatedToken::token_exists(&self.redis_key(), con) {
            redis::cmd("DEL")
                .arg(&self.redis_key())
                .execute(con)
        }
    }

    pub fn token_exists(key: &String, con: &mut redis::Connection) -> bool {
        match redis::cmd("EXISTS")
            .arg(key)
            .query(con) {
            Ok(val) => return val,
            Err(error) => panic!("Error: {:?}", error),
        }
    }

    pub fn get_token(key: &String, con: &mut redis::Connection) -> HashMap<String, String>{
        match con.hgetall(key) {
            Ok(val) => return val,
            Err(error) => panic!("Error: {:?}", error),
        }
    }

    /*
     * Getters and Setters
     */
    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn expiry_date(&self) -> &str {
        &self.expiry_date
    }

    pub fn set_token(&mut self, token: String) {
        self.token = token;
    }

    pub fn set_user_id(&mut self, user_id: String) {
        self.user_id = user_id;
    }

    pub fn set_expiry_date(&mut self, expiry_date: String) {
        self.expiry_date = expiry_date;
    }

}