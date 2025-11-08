use std::{env, sync::Mutex};

use actix_web::{App,HttpServer, web};
use dotenvy::dotenv;
 
mod controllers;
mod db;


struct AppState {
    db: Mutex<sqlx::postgres::PgPool>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap_or("Could not load database url".to_string());
    let state = web::Data::new(AppState{
        db: Mutex::new(
            sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .unwrap()
        )
    }
);
    HttpServer::new(move  || {
        App::new()
        .app_data(state.clone())
        .service(controllers::auth::sign_in)
        .service(controllers::auth::sign_up)
        .service(controllers::profile::get_me)
        .service(controllers::profile::update_me)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}