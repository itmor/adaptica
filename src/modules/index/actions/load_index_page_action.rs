
use actix_web::{Responder, HttpResponse};
use askama::Template; 
use crate::modules::index::views::IndexTemplate;

pub async fn load() -> impl Responder {
    let template = IndexTemplate {};

    match template.render() {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
