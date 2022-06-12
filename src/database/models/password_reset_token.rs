use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct PasswordResetToken{
    pub id :u64,
    pub expiryDate :DateTime<Utc>,
    pub token :String,
    pub user_id :u32,
}

use super::schema::password_reset_token;