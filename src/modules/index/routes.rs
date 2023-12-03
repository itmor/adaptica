use actix_web::web;
use super::actions::index_action;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(index_action::load)),
    );
}