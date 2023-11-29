
use actix_web::{Responder, HttpResponse};

pub async fn load_posts() -> impl Responder {
    HttpResponse::Ok().body("List of blog posts")
}
