use actix_web::{get, Responder};

#[get("/users")]
pub async fn get_users() -> impl Responder {
    format!("Hello from GET users")
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