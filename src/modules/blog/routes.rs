use actix_web::web;
use super::actions::add_post_action;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/blog")
            .route("/add_post", web::get().to(add_post_action::add_post)),
    );
}
