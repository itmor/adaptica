
use actix_web::{Responder, HttpResponse};
use sailfish::TemplateOnce;

use crate::{modules::blog::views::AddPostTemplate, shared::templates::BaseTemplate};

pub async fn add_post() -> impl Responder {
    let content = AddPostTemplate {}.render_once().unwrap();
    let base_template = BaseTemplate { 
        content: content,
        css_paths: vec![
            "/styles/base.css".to_string(), 
            "/index/styles/index.css".to_string(), 
            "/styles/content_card.css".to_string(), 
        ],
        
    };

    match base_template.render_once() {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}