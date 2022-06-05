use serde::{Serialize, Deserialize};

#[derive(Debug,Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
}