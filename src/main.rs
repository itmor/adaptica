
use actix_web::{App, HttpServer};

mod modules {
    pub mod index;
    pub mod blog;
    pub mod user;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .configure(modules::index::routes::init)
            .configure(modules::blog::routes::init)
            .configure(modules::user::routes::init)
    })
    .bind("127.0.0.2:8080")?
    .run()
    .await
}
