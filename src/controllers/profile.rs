

use actix_web::{Responder, get, post};



#[get("/me")]
pub async fn get_me() -> impl Responder{
    "Profile"
}


#[post("/me")]
pub async fn update_me() -> impl Responder{
    "Update me"
}