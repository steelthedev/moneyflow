

use actix_web::{Responder, get, post};



#[get("/me")]
pub async fn sign_up() -> impl Responder{
    "Profile"
}


#[post("/me")]
pub async fn sign_in() -> impl Responder{
    "Update me"
}