use actix_web::{get, post, delete, web, HttpResponse, Error};
use super::Pool;
use super::db;
use super::models::InputUser;

#[get("/users")]
pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db::get_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|err| HttpResponse::InternalServerError().json(format!("{:?}", err)))?)
}

#[get("/users/{id}")]
pub async fn get_user(db: web::Data<Pool>, user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db::get_user(db, user_id.into_inner()))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}

#[post("/users")]
pub async fn add_user(db: web::Data<Pool>, user: web::Json<InputUser>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db::add_user(db, user.into_inner()))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}

#[delete("/users/{id}")]
pub async fn delete_user(db: web::Data<Pool>, user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db::delete_user(db, user_id.into_inner()))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}