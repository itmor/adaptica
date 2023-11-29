
use actix_web::{web, Responder, HttpResponse};

pub async fn show_user(user_id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().body(format!("User ID: {}", user_id))
}
