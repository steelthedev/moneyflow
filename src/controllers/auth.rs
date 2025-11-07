use actix_web::{Responder, post};



#[post("/auth/sign-up")]
pub async fn sign_up() -> impl Responder{
    "sign up"
}


#[post("/auth/sign-in")]
pub async fn sign_in() -> impl Responder{
    "sign in"
}