use std::collections::HashMap;
use redis::{Commands, RedisError};
use serde::{Serialize, Deserialize};
use std::{env};

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

/*
 * Redis Methods
 */

fn create_redis_connection() -> Result<redis::Connection, RedisError>{

    let redis_host = env::var("REDIS_HOST").expect("REDIS_HOST must be set");
    let redis_port = env::var("REDIS_PORT").expect("REDIS_PORT must be set");
    let redis_user = env::var("REDIS_USER").expect("REDIS_USER must be set");
    let redis_password = env::var("REDIS_PASSWORD").expect("REDIS_PASSWORD must be set");

    let redis_url: String;
    if redis_user.is_empty() || redis_password.is_empty() {
        redis_url = format!("redis://{}:{}", redis_host, redis_port);
    }else {
        redis_url = format!("redis://{}:{}@{}:{}",redis_user, redis_password, redis_host, redis_port);
    }

    let client = redis::Client::open(redis_url)?;

    Ok(client.get_connection()?)
}

pub fn redis_key(invalid_token :&InvalidatedToken) -> String {
    format!("invalidated_token:{}", &invalid_token.token)
}

pub fn save_token(invalid_token :&InvalidatedToken) -> Result<(), RedisError> {
    let mut con= create_redis_connection().map_err(|e| e)?;
    redis::cmd("HMSET")
        .arg(redis_key(invalid_token))
        .arg(&[("token", &invalid_token.token), ("user_id", &invalid_token.user_id), ("expiry_date", &invalid_token.expiry_date)])
        .execute(&mut con);
    Ok(())
}

pub fn token_exists(key: &String) -> Result<bool , RedisError> {
    let mut con= create_redis_connection().map_err(|e| e)?;
    redis::cmd("EXISTS").arg(key).query(&mut con)
}

pub fn delete_token(invalid_token :&InvalidatedToken) -> Result<(), RedisError>{
    let mut con= create_redis_connection().map_err(|e| e)?;
    match token_exists(&redis_key(invalid_token)) {
        Ok(_) => {
            redis::cmd("DEL")
                .arg(redis_key(invalid_token))
                .execute(&mut con)
        }
        Err(_) => {}
    }

    Ok(())
}

fn get_token_hash(key: &String, con: &mut redis::Connection) -> HashMap<String, String>{
    match con.hgetall(key) {
        Ok(val) => return val,
        Err(error) => panic!("Error: {:?}", error),
    }
}

pub fn get_token(key: &String) -> Option<InvalidatedToken>{
    match create_redis_connection(){
        Ok(mut con) => {
            let hash = get_token_hash(key,&mut con);
            if hash.contains_key("token") {
                match serde_json::to_string(&hash){
                    Ok(hash_string) => {
                        match serde_json::from_str(&hash_string) {
                            Ok(token) => {Some(token)}
                            Err(_) => {None}
                        }
                    }
                    Err(_) => {None}
                }
            }else {None}
        }
        Err(_) => {None}
    }
}

pub fn get_all_tokens() -> Result<Vec<InvalidatedToken>, RedisError> {
    let mut con= create_redis_connection().map_err(|e| e)?;
    let keys = redis::cmd("keys").arg("invalidated_token:*").query::<Vec<String>>(&mut con)?;
    let mut invalidated_tokes :Vec<InvalidatedToken> = Vec::new();
    for key in keys.iter(){
        match get_token(key) {
            Some(token) => {
                invalidated_tokes.push(token);
            }
            _ => {}
        }
    }
    Ok(invalidated_tokes)
}