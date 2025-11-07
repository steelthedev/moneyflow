use actix_web::{App,HttpServer};


mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(controllers::auth::sign_in)
        .service(controllers::auth::sign_up)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}