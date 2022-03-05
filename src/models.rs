use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32, 
    pub username: String,
    pub email: String
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub username: String,
    pub email: String
}