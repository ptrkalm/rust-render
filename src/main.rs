#[macro_use]
extern crate diesel;

use actix_web::{HttpServer, App};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod routes;
mod models;
mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(routes::get_users)
            .service(routes::get_user)
            .service(routes::add_user)
            .service(routes::delete_user)
    })
    .bind(std::env::var("ADDR").expect("ADDR must be set"))?
    .run()
    .await
}