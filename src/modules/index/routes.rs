use actix_web::web;
use super::actions::index_action;
use super::actions::show_login_form_action;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(index_action::load))
           
    ).service(web::resource("/login")
    .route(web::get().to(show_login_form_action::show_login_form)));
}