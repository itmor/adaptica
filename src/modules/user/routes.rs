use actix_web::web;
use super::actions::show_user_action;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("/{id}", web::get().to(show_user_action::show_user)),
    );
}
