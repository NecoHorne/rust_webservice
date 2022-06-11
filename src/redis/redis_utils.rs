use std::env;
use redis::RedisError;
use crate::security::tokens::invalidated_token::InvalidatedToken;

pub fn create_redis_connection() -> Result<redis::Connection, RedisError>{

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

pub fn get_all_tokens(con: &mut redis::Connection) -> Vec<InvalidatedToken> {
    let keys = redis::cmd("keys").arg("invalidated_token:*").query::<Vec<String>>(con).unwrap();
    let mut invalidated_tokes :Vec<InvalidatedToken> = Vec::new();
    for key in keys.iter(){
        let hash = InvalidatedToken::get_token(key, con);
        let hash_string = serde_json::to_string(&hash).unwrap();
        let invalid_token :InvalidatedToken = serde_json::from_str(&hash_string).unwrap();
        invalidated_tokes.push(invalid_token);
    }
    return invalidated_tokes;
}