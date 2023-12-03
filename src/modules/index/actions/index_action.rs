
use actix_web::{Responder, HttpResponse};
use sailfish::TemplateOnce;

use crate::{modules::index::views::IndexTemplate, shared::templates::BaseTemplate};

pub async fn load() -> impl Responder {
    let content = IndexTemplate {}.render_once().unwrap();
    let base_template = BaseTemplate { 
        content: content,
        css_paths: vec![
            "/styles/base.css".to_string(), 
            "/index/styles/index.css".to_string(), 
        ],
        
    };

    match base_template.render_once() {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
