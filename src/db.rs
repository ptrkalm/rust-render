use actix_web::web;
use super::models::*;
use super::schema::users::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::diesel::result::Error;
use diesel::dsl::{delete, insert_into};

pub fn get_users(pool: web::Data<Pool>) -> Result<Vec<User>, Error> {
    let conn = pool.get().unwrap();
    users.load::<User>(&conn)
}

pub fn get_user(pool: web::Data<Pool>, user_id: i32) -> Result<User, Error> {
    let conn = pool.get().unwrap();
    users.find(user_id).get_result::<User>(&conn)
}

pub fn add_user(pool: web::Data<Pool>, user: InputUser) -> Result<User, Error> {
    let conn = pool.get().unwrap();
    let new_user = NewUser {
        username: &user.username,
        email: &user.email
    };
    insert_into(users).values(&new_user).get_result(&conn)
}

pub fn delete_user(pool: web::Data<Pool>, user_id: i32) -> Result<usize, Error> {
    let conn = pool.get().unwrap();
    delete(users.find(user_id)).execute(&conn)
}