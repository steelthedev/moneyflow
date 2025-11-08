use std::time::SystemTime;

use actix_web::{HttpResponse, Responder, post, web};
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::json;

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


#[derive(Deserialize, Debug)]
pub struct SignInRequest{
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims{
    pub sub: i64,
    pub role: String,
    pub exp: u64,
}

#[post("/auth/sign-in")]
pub async fn sign_in(state: web::Data<AppState>, data: web::Json<SignInRequest>) -> Result<HttpResponse, actix_web::Error>{
    let db = state.db.lock().unwrap();

    let user = match db::user::get_by_email(&db, &data.email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(json!({
                "message": "user not found",
                "status": 404
            })));
        },
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "message": "database error",
                "status": 500
            })));
        }
    };

    if !bcrypt::verify(&data.password, &user.password).unwrap(){
        return Ok(HttpResponse::Unauthorized().json(json!({
            "message":"Invalid password",
            "status": 401,
        })));
    }

   let claims = Claims{
    sub: user.id,
    role: "user".to_string(),
    exp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() + 4  * 60 * 60,
   };

   let token = jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(state.secret_key.as_bytes()))
       .map_err(actix_web::error::ErrorInternalServerError)?;

   Ok(HttpResponse::Ok().json(json!(
    {
        "message":"User logged in successfully",
        "token": token,
        "status": 200,
    }
   )))
}