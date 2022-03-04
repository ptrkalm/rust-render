use actix_web::*;
mod routes;

#[get("/")]
async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(routes::get_users)
            .service(routes::get_user)
            .service(routes::add_user)
            .service(routes::delete_user)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}