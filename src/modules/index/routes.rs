use actix_web::web;
use super::actions::load_posts_action;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(load_posts_action::load_posts)),
    );
}