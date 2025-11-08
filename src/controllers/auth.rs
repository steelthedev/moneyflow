use actix_web::{HttpResponse, Responder, post, web};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::broadcast::error;

use crate::{AppState, db::{self, user::create_user}};


#[derive(Deserialize, Debug)]
pub struct SignUpRequest{
   pub  email: String,
    pub password:String,
    pub firstname:String,
    pub lastname: String,
}


#[post("/auth/sign-up")]
pub async fn sign_up(state: web::Data<AppState>, data: web::Json<SignUpRequest>) -> Result<HttpResponse, actix_web::Error>{
    let db = state.db.lock().unwrap();

    if db::user::has_with_email(&db, &data.email).await.map_err(actix_web::error::ErrorInternalServerError)?{
        return Ok(HttpResponse::BadRequest().body("Email already exists"));
    }

    let user_id = db::user::create_user(&db, &data).await.map_err(actix_web::error::ErrorInternalServerError)?;

   Ok(HttpResponse::Ok().json(json!(
    {
        "message":"user created successfully",
        "user_id": user_id,
    }
   )))
}


#[post("/auth/sign-in")]
pub async fn sign_in() -> impl Responder{
    "sign in"
}