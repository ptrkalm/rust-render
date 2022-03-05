use actix_web::{get, web, HttpResponse, Error, Responder};
use super::models::User;
use super::schema::users::dsl::*;
use super::Pool;

use crate::diesel::RunQueryDsl;

#[get("/users")]
pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
}

#[get("/users/{id}")]
pub async fn get_user() -> impl Responder {
    format!("Hello from GET user")
}

#[get("/users/add/{id}")]
pub async fn add_user() -> impl Responder {
    format!("Hello from ADD user")
}

#[get("/users/delete/{id}")]
pub async fn delete_user() -> impl Responder {
    format!("Hello from DELETE user")
}